# API Testing Framework

This document provides an overview of the testing framework for the API modules in this project.

## Directory Structure

```
Api/
├── Makefile.tests          # Master Makefile for API testing
├── README.tests.md         # This file
├── test-framework/         # Common test framework resources
│   ├── README.md           # Framework documentation
│   └── templates/          # Templates for creating new test suites
│       ├── Makefile.template
│       ├── mock_template.c
│       ├── mock_template.h
│       ├── README.md.template
│       └── test_template.c
└── <module-name>/         # e.g., isis-ants-api, isis-trxvu-api, etc.
    └── tests/             # Test directory for each module
        ├── Makefile
        ├── mock_*.c
        ├── mock_*.h
        ├── README.md
        └── test_*.c
```

## Quick Start

### Running Tests

To run tests for all API modules:

```bash
cd /home/user/NewProject/Api
make -f Makefile.tests
```

To run tests for a specific module:

```bash
cd /home/user/NewProject/Api
make -f Makefile.tests isis-ants-api
```

### Creating Tests for a New Module

To create a new test structure for an API module:

```bash
cd /home/user/NewProject/Api
make -f Makefile.tests create-test-structure MODULE=your-module-name
```

This will create a tests directory with template files that you can customize for your module.

### Generating Coverage Reports

To generate coverage reports for all modules:

```bash
cd /home/user/NewProject/Api
make -f Makefile.tests coverage
```

To generate a combined test report:

```bash
cd /home/user/NewProject/Api
make -f Makefile.tests report
```

## Test Structure

Each API module's test directory contains:

1. **Test Files**: C files containing test cases using the CMocka framework
2. **Mock Files**: Mock implementations of hardware interfaces (I2C, UART, SPI, etc.)
3. **Makefile**: Configuration for building and running tests
4. **README**: Documentation specific to testing that module

## Prerequisites

To run the tests, you need:

1. **CMocka**: `sudo apt-get install libcmocka-dev`
2. **GCC**: `sudo apt-get install gcc`
3. **Make**: `sudo apt-get install make`

## Writing Tests

When writing tests for your API module:

1. Focus on testing the API's behavior, not the implementation details
2. Test both success and error paths
3. Mock external dependencies appropriately
4. Aim for high code coverage
5. Group tests logically by functionality

## Test Organization

Each test file should include:

1. **Setup/Teardown Functions**: For test environment initialization and cleanup
2. **Test Groups**: Tests organized by functionality
3. **Mocked Functions**: Implementations that replace hardware-dependent code
4. **Test Data**: Sample data for inputs and expected outputs

## Mock Implementation

The mock implementation should:

1. Track function calls
2. Verify expected parameters
3. Return predefined values/data
4. Simulate error conditions

## Adding a New Module to the Framework

1. Create the test structure with `make -f Makefile.tests create-test-structure MODULE=your-module-name`
2. Customize the template files for your specific module
3. Add your module to the `API_MODULES` list in `Makefile.tests` 