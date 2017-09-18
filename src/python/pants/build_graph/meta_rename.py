# coding=utf-8
# Copyright 2014 Pants project contributors (see CONTRIBUTORS.md).
# Licensed under the Apache License, Version 2.0 (see LICENSE).

# Try: `./pants meta-rename --from=src/scala/org/pantsbuild/zinc/analysis:analysis --to=src/scala/org/pantsbuild/zinc/analysis:new_analysis`
# incomplete though - have to rename the directory, because for example, following the above, below (the reverse operation) would not work:
# `./pants meta-rename --from=src/scala/org/pantsbuild/zinc/analysis:new_analysis --to=src/scala/org/pantsbuild/zinc/analysis:analysis`

# QUESTIONS
# Is there a pants remove dependency function?
# are BUILD files created manually?
# if we're using the GO tool from bazelbuild - are we coding this in GO?

# Should we have an option to specify what kind of target we're changing?

# Do we prefer "dependant" or "dependee"? for a dependency's parent?

# CONSIDERATIONS
# Consider refactoring since dependees and meta_rename would use very similar code

# TODO lint and test
# TODO add docs for each method
# TODO update the BUILD file for where this task ends up
# TODO Have to do read and writes similar to build_file_manipulator class?

from __future__ import (absolute_import, division, generators, nested_scopes, print_function,
                        unicode_literals, with_statement)

from collections import defaultdict

# TODO remove pdb and pprint
# import pdb
# import pprint

# TODO: import the other target libraries. Switch between them with a type option
from pants.backend.jvm.targets.scala_library import ScalaLibrary
from pants.build_graph.address import Address
from pants.base.specs import DescendantAddresses
from pants.task.task import Task

class MetaRename(Task):
  """Rename a target for its dependees"""

  @classmethod
  def register_options(cls, register):
    super(MetaRename, cls).register_options(register)
    register('--from', type=str, advanced=True, default=None, help='The old dependency name to change')
    register('--to', type=str, advanced=True, default=None, help='The new name for the dependency')
    # TODO register the scope option
    # TODO register the type of target, scalalibrary, pythonlibrary, etc.

  def __init__(self, *args, **kwargs):
    super(MetaRename, self).__init__(*args, **kwargs)
    
    self._from = self.get_options()['from']
    self._to = self.get_options().to

  def execute(self):
    self.change_build_names()
  
  def change_build_names(self):    
    # TODO considerations: create a build_graph? TODO kwargs necessary as the fourth argument?
    from_address = Address.parse(self._from)
    to_address = Address.parse(self._to)
    self.change_build_name(from_address, from_address.target_name, to_address.target_name)

    from_target = ScalaLibrary(name=from_address.target_name, 
                               address=from_address,
                               build_graph=[],
                               **{})

    dependency_graph = self.dependency_graph()
    dependee_addresses = dependency_graph[from_target]
    for address in dependee_addresses:
      self.change_build_name(address, from_address.target_name, to_address.target_name)

  # default scope is global, ''
  def dependency_graph(self, scope=''):
    dependency_graph = defaultdict(set)
    for address in self.context.build_graph.inject_specs_closure([DescendantAddresses(scope)]):
      target = self.context.build_graph.get_target(address)
      for dependency in target.dependencies:
        dependency_graph[dependency].add(address)
    return dependency_graph

  # TODO: limit this renaming to names/dependencies so that it doesn't rename more than it should
  def change_build_name(self, address, old_name, new_name):
    print("address: {}, old_name: {}, new_name: {}".format(address, old_name, new_name))
    # TODO print errors for a bad dependency name that does not exist
    build_file = address.spec_path + '/BUILD'
    # Better way than overwriting the BUILD file?
    with open(build_file, 'r') as f:
      source = f.read()

    new_source = source.replace(old_name, new_name)

    with open(build_file, 'w') as new_build:
      new_build.write(new_source)

    # TODO come up with a concise message to denote that the address name changed
    # typical to print this success message? something like:
    # print('build_file ' + build_file)