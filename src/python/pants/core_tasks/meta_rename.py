# coding=utf-8
# Copyright 2014 Pants project contributors (see CONTRIBUTORS.md).
# Licensed under the Apache License, Version 2.0 (see LICENSE).

# QUESTIONS
# is there a pants remove dependency function?
# are BUILD files created manually?
# if we're using the GO tool from bazelbuild - are we coding this in GO?

# should we have an option to specify what kind of target we're changing?

# preferr dependant or dependee? for a dependency's parent?

# Consider refactoring since dependees and meta_rename would use very similar code
# include this as a consideration in a follow up to the PR

# TODO lint and test

from __future__ import (absolute_import, division, generators, nested_scopes, print_function,
                        unicode_literals, with_statement)

from collections import defaultdict

# TODO remove pdb and pprint
import pdb
import pprint

# bulldozer rename tool
# https://github.com/bazelbuild/buildtools/tree/master/buildozer

# TODO: import the other target libraries. Switch between them with a type option
from pants.backend.jvm.targets.scala_library import ScalaLibrary
from pants.build_graph.address import Address
from pants.base.specs import DescendantAddresses
from pants.task.task import Task

# TODO would it be useful to instantiate a dependees or dependencies class?
# Example dependees: ./pants dependees src/scala/org/pantsbuild/zinc/analysis
# a full example would be ./pants meta-rename --from=src/scala/org/pantsbuild/zinc/analysis --to=src/scala/org/pantsbuild/zinc/new_analysis
class MetaRename(Task):
  """Rename a target for its dependees"""

  # change the name of a target wherever it's listed as a dependency

  # TODO should the default be None? what is advanced?
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
    self.change_dependency_names()
  
  def change_dependency_names(self):    
    # TODO considerations: create a build_graph? TODO kwargs necessary as the fourth argument?
    address=Address.parse(self._from)
    from_root = ScalaLibrary(name=address.target_name, 
                               address=address,
                               build_graph=[],
                               **{})

    dependency_graph = self.dependency_graph()
    dependees = self.get_dependees(dependency_graph, [from_root])
    deps = defaultdict(list)
    for dependee in dependees:
      self.change_dependency_name(dependee, self._from, self._to)

  # default scope is global
  def dependency_graph(self, scope=''):
    dependency_graph=defaultdict(set)
    for address in self.context.build_graph.inject_specs_closure([DescendantAddresses(scope)]):
      target = self.context.build_graph.get_target(address)
      for dependency in target.dependencies:
        dependency_graph[dependency].add(target)
    return dependency_graph

  def get_dependees(self, dependency_graph, roots):
    known_dependents = set()
    for target in roots:
      known_dependents.update(dependency_graph[target])
    return known_dependents

  def change_dependency_name(self, dependency, old_name, new_name):
    # rename old_name to new_name in dependency
    # TODO come up with a concise message to denote that the dependency name changed
    print("dependency: {}, old_name: {}, new_name".format(dependency, old_name, new_name))

    # here's the add
    # new_name is a str though. Will this work?
    # dependency.add(new_name)

    # pdb.set_trace()
