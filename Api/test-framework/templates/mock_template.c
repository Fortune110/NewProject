/*
 * Mock Interface Implementation for TEMPLATE_MODULE API Tests
 * Copyright (C) 2023
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#include <stdarg.h>
#include <stddef.h>
#include <setjmp.h>
#include <string.h>
#include <cmocka.h>
#include <stdio.h>
#include "mock_template_module.h"

/*--------------------------------------------------
 * Mock Function Call Counters
 *--------------------------------------------------*/

/* Define counters for each function call */
static struct {
    int i2c_open;
    int i2c_close;
    int i2c_write;
    int i2c_read;
    int uart_open;
    int uart_close;
    int uart_write;
    int uart_read;
    int spi_open;
    int spi_close;
    int spi_transfer;
    /* Add counters for other functions as needed */
} call_counts;

/*--------------------------------------------------
 * Mock Interface Initialization
 *--------------------------------------------------*/

void mock_init(void) {
    /* Reset all call counters */
    mock_reset_counters();
}

void mock_cleanup(void) {
    /* Nothing to do in this simple example */
}

void mock_reset_counters(void) {
    memset(&call_counts, 0, sizeof(call_counts));
}

/*--------------------------------------------------
 * Mock Call Counter Functions
 *--------------------------------------------------*/

bool mock_function_called(const char *function_name, int expected_calls) {
    if (strcmp(function_name, "i2c_open") == 0) {
        return call_counts.i2c_open == expected_calls;
    } else if (strcmp(function_name, "i2c_close") == 0) {
        return call_counts.i2c_close == expected_calls;
    } else if (strcmp(function_name, "i2c_write") == 0) {
        return call_counts.i2c_write == expected_calls;
    } else if (strcmp(function_name, "i2c_read") == 0) {
        return call_counts.i2c_read == expected_calls;
    } else if (strcmp(function_name, "uart_open") == 0) {
        return call_counts.uart_open == expected_calls;
    } else if (strcmp(function_name, "uart_close") == 0) {
        return call_counts.uart_close == expected_calls;
    } else if (strcmp(function_name, "uart_write") == 0) {
        return call_counts.uart_write == expected_calls;
    } else if (strcmp(function_name, "uart_read") == 0) {
        return call_counts.uart_read == expected_calls;
    } else if (strcmp(function_name, "spi_open") == 0) {
        return call_counts.spi_open == expected_calls;
    } else if (strcmp(function_name, "spi_close") == 0) {
        return call_counts.spi_close == expected_calls;
    } else if (strcmp(function_name, "spi_transfer") == 0) {
        return call_counts.spi_transfer == expected_calls;
    }
    
    /* Function not found */
    printf("Warning: Unknown function '%s' in mock_function_called\n", function_name);
    return false;
}

/*--------------------------------------------------
 * Mock I2C Interface Implementation
 *--------------------------------------------------*/

int __wrap_i2c_open(const char *bus, uint16_t addr) {
    call_counts.i2c_open++;
    
    /* Return value set by the test case */
    return mock_type(int);
}

int __wrap_i2c_close(int fd) {
    call_counts.i2c_close++;
    
    /* Usually returns success */
    return 0;
}

int __wrap_i2c_write(int fd, uint8_t cmd, const uint8_t *data, size_t data_len) {
    call_counts.i2c_write++;
    
    /* Check expected parameters */
    check_expected(cmd);
    check_expected(data_len);
    
    if (data && data_len > 0) {
        check_expected_ptr(data);
    }
    
    /* Return value set by the test case */
    return mock_type(int);
}

int __wrap_i2c_read(int fd, uint8_t cmd, uint8_t *rx_buffer, size_t rx_len) {
    call_counts.i2c_read++;
    
    /* Check expected parameters */
    check_expected(cmd);
    check_expected(rx_len);
    
    /* If a buffer is provided, copy the mock data */
    if (rx_buffer && rx_len > 0) {
        uint8_t *mock_data = mock_ptr_type(uint8_t *);
        if (mock_data) {
            memcpy(rx_buffer, mock_data, rx_len);
        }
    }
    
    /* Return value set by the test case */
    return mock_type(int);
}

/*--------------------------------------------------
 * Mock UART Interface Implementation
 *--------------------------------------------------*/

int __wrap_uart_open(const char *device, int baudrate) {
    call_counts.uart_open++;
    
    /* For most tests, return a positive file descriptor */
    return mock_type(int);
}

int __wrap_uart_close(int fd) {
    call_counts.uart_close++;
    
    /* Usually returns success */
    return 0;
}

int __wrap_uart_write(int fd, const uint8_t *data, size_t len) {
    call_counts.uart_write++;
    
    check_expected(len);
    
    if (data && len > 0) {
        check_expected_ptr(data);
    }
    
    /* Return value set by the test case */
    return mock_type(int);
}

int __wrap_uart_read(int fd, uint8_t *buffer, size_t len, int timeout_ms) {
    call_counts.uart_read++;
    
    check_expected(len);
    check_expected(timeout_ms);
    
    /* If a buffer is provided, copy the mock data */
    if (buffer && len > 0) {
        uint8_t *mock_data = mock_ptr_type(uint8_t *);
        if (mock_data) {
            size_t mock_len = mock_type(size_t);
            size_t copy_len = (len < mock_len) ? len : mock_len;
            memcpy(buffer, mock_data, copy_len);
            return copy_len;
        }
    }
    
    /* Return value set by the test case */
    return mock_type(int);
}

/*--------------------------------------------------
 * Mock SPI Interface Implementation
 *--------------------------------------------------*/

int __wrap_spi_open(const char *device, int mode, int speed_hz) {
    call_counts.spi_open++;
    
    /* Return value set by the test case */
    return mock_type(int);
}

int __wrap_spi_close(int fd) {
    call_counts.spi_close++;
    
    /* Usually returns success */
    return 0;
}

int __wrap_spi_transfer(int fd, const uint8_t *tx_buf, uint8_t *rx_buf, size_t len) {
    call_counts.spi_transfer++;
    
    check_expected(len);
    
    if (tx_buf && len > 0) {
        check_expected_ptr(tx_buf);
    }
    
    /* If a rx buffer is provided, copy the mock data */
    if (rx_buf && len > 0) {
        uint8_t *mock_data = mock_ptr_type(uint8_t *);
        if (mock_data) {
            memcpy(rx_buf, mock_data, len);
        }
    }
    
    /* Return value set by the test case */
    return mock_type(int);
}

/*--------------------------------------------------
 * API-Specific Mock Functions
 *--------------------------------------------------*/

/* Add any API-specific mock functions here */ 