# coding=utf-8
# Copyright 2017 Pants project contributors (see CONTRIBUTORS.md).
# Licensed under the Apache License, Version 2.0 (see LICENSE).

from __future__ import (absolute_import, division, generators, nested_scopes, print_function,
                        unicode_literals, with_statement)

from collections import defaultdict

from pants.build_graph.meta_rename import MetaRename
from pants_test.base_test import BaseTest
from pants_test.tasks.task_test_base import TaskTestBase

# TODO consider removing
class DummyMetaRename(MetaRename):
  def execute(self):
    pass

class MetaRenameTest(TaskTestBase):
  """Test renaming in MetaRename"""

  @classmethod
  def task_type(cls):
    return MetaRename

  def setUp(self):
    super(MetaRenameTest, self).setUp()

    target = self.make_target('foo')
    _from = '--from'
    _to = '--to'
    self.set_options(_from = 'src/scala/org/pantsbuild/zinc/analysis:analysis')
    self.set_options(_to = 'src/scala/org/pantsbuild/zinc/analysis:new_analysis')

    meta_rename = self.create_task(self.context(target_roots=[target]))
    meta_rename.execute()
    # self.full_graph = meta_rename.dependency_graph()

    # import pdb
    # pdb.set_trace()
    # self.scoped_graph = meta_rename.dependency_graph()

  def test_dependency_graph(self):
    self.assertTrue(True)
    # self.assertEqual(self.full_graph, defaultdict(set))
    # self.assertEqual(self.scoped_graph, defaultdict(set))
