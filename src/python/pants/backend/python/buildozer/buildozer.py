# TODO: move buildozer.py and test_buildozer.py to the contrib module

# coding=utf-8
# Copyright 2017 Pants project contributors (see CONTRIBUTORS.md).
# Licensed under the Apache License, Version 2.0 (see LICENSE).

from __future__ import (absolute_import, division, generators, nested_scopes, print_function,
                        unicode_literals, with_statement)


from pants.base.build_environment import get_buildroot
from pants.base.exceptions import TaskError
from pants.binaries.binary_util import BinaryUtil
from pants.task.task import Task
from pants.util.process_handler import subprocess


class Buildozer(Task):
  """Enables interaction with the Buildozer Go binary
  
  Behavior:
  1. `./pants buildozer --add-dependencies=<dependencies> --location=<directory>` 
      will add the dependency to the location's relative BUILD file.
  2. `./pants buildozer --remove-dependencies=<dependencies> --location=<directory>` 
      will remove the dependency from the location's relative BUILD file.

  Note that buildozer assumes that BUILD files contain a name field for the target.

  # TODO: put example of multiple dependencies
  """

  @classmethod
  def register_options(cls, register):
    # TODO should the version be not advanced?
    # TODO is the default not advanced anyway?
    register('--version', advanced=False, fingerprint=True, default='0.4.5', help='Version of buildozer.')
    register('--add-dependencies', type=str, advanced=False, default=None, help='The dependency or dependencies to add')
    register('--remove-dependencies', type=str, advanced=False, default=None, help='The dependency or dependencies to remove')
    # register('--location', type=str, advanced=True, default=None, help='The target location')

  def __init__(self, *args, **kwargs):
    super(Buildozer, self).__init__(*args, **kwargs)

    self.options = self.get_options()
    self.address = self.context.target_roots[0].address
    # self._executable = BinaryUtil.Factory.create().select_script('scripts/buildozer', self.options.version, 'buildozer')
    
  def execute(self):
    # import pdb
    # pdb.set_trace()

    if self.options.add_dependencies:
      self.add_dependencies()

    if self.options.remove_dependencies:
      self.remove_dependencies()

  def add_dependencies(self):
    # import pdb
    # pdb.set_trace()

    self._execute_buildozer_script('add dependencies {}'.format(self.options.add_dependencies))
  
  def remove_dependencies(self):
    self._execute_buildozer_script('remove dependencies {}'.format(self.options.remove_dependencies))

  def _execute_buildozer_script(self, command):
    # buildozer_command = [self._executable, command]
    # import pdb
    # pdb.set_trace()

#     (Pdb) self.context.target_roots[0].address._spec_path
# 'tmp'
# (Pdb) self.context.target_roots[0].address._target_naem
# *** AttributeError: 'BuildFileAddress' object has no attribute '_target_naem'
# (Pdb) self.context.target_roots[0].address._target_name
    # [u'/Users/davidkatz/buildozer', u'add dependencies /a/b/c']
    # how to turn this into a usable string for target_roots
    buildozer_command = [
      '/Users/davidkatz/buildozer', command
    ]

    # import pdb
    # pdb.set_trace()

    # if self.options.get('location'):
    #   buildozer_command.append(self.options.location)

    try:
      subprocess.check_call(buildozer_command, cwd=get_buildroot())
    except subprocess.CalledProcessError as err:
      if (err.returncode == 3):
        raise TaskError('{} ... no changes were made'.format(buildozer_command))
      else:
        raise TaskError('{} ... exited non-zero ({}).'.format(buildozer_command, err.returncode))
