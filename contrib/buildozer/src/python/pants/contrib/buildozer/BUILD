# Copyright 2017 Pants project contributors (see CONTRIBUTORS.md).
# Licensed under the Apache License, Version 2.0 (see LICENSE).

python_library(
  name='plugin',
  sources=['register.py'],
  dependencies=[
    ':buildozer',
  ],
  provides=contrib_setup_py(
    name='pantsbuild.pants.contrib.buildozer',
    description='Buildozer tool to manipulate and edit BUILD files',
  )
)


python_library(
  name='buildozer',
  sources=['buildozer.py'],
  dependencies = [
    'src/python/pants/goal:task_registrar',
  ]
)
