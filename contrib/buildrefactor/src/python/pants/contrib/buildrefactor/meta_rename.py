# coding=utf-8
# Copyright 2014 Pants project contributors (see CONTRIBUTORS.md).
# Licensed under the Apache License, Version 2.0 (see LICENSE).

from __future__ import (absolute_import, division, generators, nested_scopes, print_function,
                        unicode_literals, with_statement)

from collections import defaultdict

from pants.backend.jvm.targets.scala_library import ScalaLibrary
from pants.base.specs import DescendantAddresses
from pants.build_graph.address import Address
from pants.contrib.buildrefactor.buildozer import Buildozer
from pants.task.task import Task

class MetaRename(Task):
  """Rename a target for its dependants

  Provides a mechanism for renaming the target's name within its local BUILD file.
  Also renames the target for its addresses wherever it's specified as a dependency.
  """

  @classmethod
  def register_options(cls, register):
    super(MetaRename, cls).register_options(register)

    register('--from', type=str, advanced=True, default=None, help='The old dependency name to change')
    register('--to', type=str, advanced=True, default=None, help='The new name for the dependency')

  def __init__(self, *args, **kwargs):
    super(MetaRename, self).__init__(*args, **kwargs)

    self._from = Address.parse(self.get_options()['from'])
    # TODO: Address.parse on this line
    self._to = self.get_options().to

  def execute(self):
    # change name in original build file
    self.update_original_build()
    self.update_target_builds()

  def update_original_build(self):
    print("use set name: {}".format(self._from))

    # how to initialize...?
    # how to set/pass in the options?
    # buildozer = Buildozer((self._from), {'options_scope': {'command': 'foo'}})
    buildozer = Buildozer((self._from), {'options_scope': []})
    # self.create_task(self.context(target_roots=self.targets['b'])).execute()
        # return type(subclass_name, (task_type,), {'_stable_name': task_type._compute_stable_name(),
                                              # 'options_scope': options_scope})
    # buildozer.context.target_roots = self._from

  def update_target_builds(self):
    # from_address = Address.parse(self._from)
    # to_address = Address.parse(self._to)

    # self.replace_in_file(from_address.spec_path + '/BUILD', from_address.target_name, to_address.target_name)

    dependency_graph = self.dependency_graph()
    dependant_addresses = dependency_graph[
      ScalaLibrary(name=self._from.target_name, address=self._from, build_graph=[], **{})
    ]

    for address in dependant_addresses:
      # remove old dependency
      # add new dependency
      # import pdb
      # pdb.set_trace()
      # self.replace_in_file(address.spec_path + '/BUILD', from_address.target_name, to_address.target_name)

      # if address.target_name == self._from:
      #   print("use set name")
      # else:
      print("use attr replace on {}".format(address))
      # if the address is the original address, rename the name field
        # ./buildozer 'set name new_tmp' //tmp:tmp
      # else

      # remove the old dependency and add the new one with buildozer

      # replace <attr> <old_value> <new_value>: Replaces old_value with new_value in the list attr. Wildcard * matches all attributes. Lists not containing old_value are not modified.

  def dependency_graph(self, scope=''):
    dependency_graph = defaultdict(set)

    for address in self.context.build_graph.inject_specs_closure([DescendantAddresses(scope)]):
      target = self.context.build_graph.get_target(address)

      for dependency in target.dependencies:
        dependency_graph[dependency].add(address)

    return dependency_graph

  def replace_in_file(self, _file, old_name, new_name):
    with open(_file, 'r') as f:
      source = f.read()

    new_source = source.replace(old_name, new_name)

    with open(_file, 'w') as new_file:
      new_file.write(new_source)
