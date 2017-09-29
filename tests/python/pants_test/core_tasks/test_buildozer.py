# coding=utf-8
# Copyright 2017 Pants project contributors (see CONTRIBUTORS.md).
# Licensed under the Apache License, Version 2.0 (see LICENSE).

from __future__ import (absolute_import, division, generators, nested_scopes, print_function,
                        unicode_literals, with_statement)

from pants.backend.jvm.targets.java_library import JavaLibrary
from pants.build_graph.build_file_aliases import BuildFileAliases
from pants.core_tasks.buildozer import Buildozer
from pants_test.tasks.task_test_base import TaskTestBase

class BuildozerTest(TaskTestBase):
  """Test the buildozer tool"""

  @property
  def alias_groups(self):
    return BuildFileAliases(targets={ 'java_library': JavaLibrary })

  @classmethod
  def task_type(cls):
    return Buildozer

  def setUp(self):
    super(BuildozerTest, self).setUp()

    self.targets = self._prepare_dependencies()

  # test add
  # test add on adding dependencies
  # have a helper function to parse the build files for depenencies
  # assert that the dependency was added

  def test_add_dependency(self):
    mock_dependency = '/a/b/c'

    # way to programmatically get the right directory location?
    # TODO oneline
    # location = './' 
    # set an add option with the mock dependency

    # where is the BUILD file?
    # look at how create_library is implemented

    # how to get the relative directory
    self.set_options(**{ 'add': mock_dependency, 'location': './' })

    buildozer_task = self.create_task(self.context(target_roots=self.targets))
    buildozer_task.execute()

    # testing TODO: remove
    with open('./BUILD', 'r') as f:
      source = f.read()

    # testing TODO: remove
    # look for a working build file directory
    import pdb
    pdb.set_trace()

    # self assert that it was actually added
    self.assertTrue(True)

  # test remove

  # test custom command

  # test that custom -help was executed without error ?

  # need to be able to import the Go Binary in the testing environment

  def _prepare_dependencies(self):
    targets = {}

    targets['a'] = self.create_library('a', 'java_library', 'a', ['A.java'])
    targets['b'] = self.create_library('b', 'java_library', 'b', ['B.java'], dependencies=['a'])

    return targets.values()


# *********

# ./pants test tests/python/pants_test/core_tasks:buildozer

# TODO
# split the modification of the contrib into a separate commit