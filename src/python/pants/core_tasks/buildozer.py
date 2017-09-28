# coding=utf-8
# Copyright 2016 Pants project contributors (see CONTRIBUTORS.md).
# Licensed under the Apache License, Version 2.0 (see LICENSE).

from __future__ import (absolute_import, division, generators, nested_scopes, print_function,
                        unicode_literals, with_statement)
import subprocess

from pants.backend.go.targets.go_binary import GoBinary
from pants.base.exceptions import TaskError
from pants.binaries.binary_util import BinaryUtil
from pants.task.task import Task

class Buildozer(Task):
  """Enables interaction with the Buildozer Go binary
  
  Behavior:
  1. `./pants buildozer --add=<dependency> --location=<directory>` 
      will add the dependency to the location's relative BUILD file.
  2. `./pants buildozer --remove=<dependency> --location=<directory>` 
      will remove the dependency from the location's relative BUILD file.
  3. `./pants buildozer --command=<custom-command>` 
      will execute a custom buildozer command

  Example: `./pants buildozer --add=a/b/c --location=//tmp:tmp`
  """

  @classmethod
  def register_options(cls, register):
      register('--add', type=str, advanced=True, default=None, help='The dependency to add')
      register('--remove', type=str, advanced=True, default=None, help='The dependency to remove')
      register('--location', type=str, advanced=True, default=None, help='The target location')
      register('--command', type=str, advanced=True, default=None, help='Custom buildozer command')
      # TODO add advanced command option for an advanced buildozer user

  def __init__(self, *args, **kwargs):
    super(Buildozer, self).__init__(*args, **kwargs)
    # necessary?
    self.options = self.get_options()
    
  def execute(self):
    # if no location specified, raise a warning

    if self.options.add:
      self.add_dependency()

    if self.options.remove:
      self.remove_dependency()

    if self.options.command:
      self.execute_custom_command()

  def add_dependency(self):
    self.execute_buildozer_script('add dependencies ' + self.options.add, self.options.location)
  
  def remove_dependency(self):
    self.execute_buildozer_script('remove dependencies ' + self.options.remove, self.options.location)

  def execute_custom_command(self):
    self.execute_buildozer_script(self.options.command)

  
  def execute_buildozer_script(self, command, directory=None):
    # TODO: include in PR description - replace the binary with the fetched image or one on the pants repo
    # what is the working directory when you run a Pants process?
    
    # shell option is needed for permissions
    # TODO: research a different option
    buildozer_command = '/Users/davidkatz/buildozer \'{}\'{}'.format(command, ' ' + directory if directory else '')

    try:
       subprocess.Popen(buildozer_command, shell=True)
    except subprocess.CalledProcessError as err:
      raise TaskError('{} ... exited non-zero ({}).').format(buildozer_command, err.returncode)
    


# NOTES | DIRTY| TODO REMOVE ******************

# add an option to specify a full buildozer command string

# how to get over the name hurdle? Where buildozer assumes a name is present in the BUILD file
# even though pants doesn't
  # hack a go solution in buildozer?
  # if I don't find the name - take the name of the directory

# buildozer-command option


# example: use tmp/BUILD

# example add:
#  ./pants buildozer --add="e/d" --location="//tmp:tmp"

# example remove: