# coding=utf-8
# Copyright 2016 Pants project contributors (see CONTRIBUTORS.md).
# Licensed under the Apache License, Version 2.0 (see LICENSE).

from __future__ import (absolute_import, division, generators, nested_scopes, print_function,
                        unicode_literals, with_statement)
import os

from pants.backend.go.targets.go_binary import GoBinary
from pants.binaries.binary_util import BinaryUtil
from pants.task.task import Task

class Buildozer(Task):
  """Enables interaction with the Buildozer Go binary
  
  Behavior:
  1. `./pants buildozer --help` will show the list of buildozer commands
  """

  def __init__(self, *args, **kwargs):
    super(Buildozer, self).__init__(*args, **kwargs)
    # necessary?
    self.options = self.get_options()

  # @classmethod
  # def register_options(cls, register):
  #     # register('--help', )

  def execute(self):
    buildozer_script = BinaryUtil.Factory.create().select_script('./scripts/buildozer')