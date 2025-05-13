# API Test Framework

This directory contains a common test framework for all API modules in the project. It provides templates and utilities to create consistent and comprehensive unit tests for hardware interfaces.

## Overview

The test framework includes:

1. **Templates**: Standard template files for creating new test suites
2. **Master Makefile**: Located at `/Api/Makefile.tests` to build and run tests for all API modules

## Creating Tests for a New API Module

To create a test suite for a new API module, run:

```bash
cd /home/user/NewProject/Api
make -f Makefile.tests create-test-structure MODULE=your-module-name
```

This will create the following structure:

```
your-module-name/
└── tests/
    ├── Makefile
    ├── mock_your_module_name.c
    ├── mock_your_module_name.h
    ├── test_your_module_name.c
    └── README.md
```

Then customize these files for your specific module.

## Testing All Modules

To build and run tests for all API modules:

```bash
cd /home/user/NewProject/Api
make -f Makefile.tests
```

To clean all test artifacts:

```bash
make -f Makefile.tests clean
```

To generate coverage reports for all modules:

```bash
make -f Makefile.tests coverage
```

To generate a combined test report:

```bash
make -f Makefile.tests report
```

## Templates

The `templates` directory contains the following files:

- `Makefile.template`: Template Makefile for building and running tests
- `test_template.c`: Template for test implementation
- `mock_template.h`: Template for mock interface header
- `mock_template.c`: Template for mock interface implementation
- `README.md.template`: Template README for test directories

## Adding a New Module to the Test Framework

To add a new module to the framework, edit `/Api/Makefile.tests` and add your module name to the `API_MODULES` list. 