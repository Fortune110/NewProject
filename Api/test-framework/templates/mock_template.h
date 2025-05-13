/*
 * Mock Interface for TEMPLATE_MODULE API Tests
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

#ifndef MOCK_TEMPLATE_MODULE_H
#define MOCK_TEMPLATE_MODULE_H

#include <stdint.h>
#include <stdbool.h>
#include <stddef.h>

/*--------------------------------------------------
 * Mock Interface Initialization
 *--------------------------------------------------*/

/* Initialize the mock system */
void mock_init(void);

/* Clean up the mock system */
void mock_cleanup(void);

/*--------------------------------------------------
 * Mock Call Counters
 *--------------------------------------------------*/

/* Check if a specific function was called a certain number of times */
bool mock_function_called(const char *function_name, int expected_calls);

/* Reset all call counters */
void mock_reset_counters(void);

/*--------------------------------------------------
 * Mock Communication Interface Functions
 *--------------------------------------------------*/

/* 
 * Mock functions for your communication layer.
 * Examples for common interfaces:
 */

/* I2C Interface */
int __wrap_i2c_open(const char *bus, uint16_t addr);
int __wrap_i2c_close(int fd);
int __wrap_i2c_write(int fd, uint8_t cmd, const uint8_t *data, size_t data_len);
int __wrap_i2c_read(int fd, uint8_t cmd, uint8_t *rx_buffer, size_t rx_len);

/* UART Interface */
int __wrap_uart_open(const char *device, int baudrate);
int __wrap_uart_close(int fd);
int __wrap_uart_write(int fd, const uint8_t *data, size_t len);
int __wrap_uart_read(int fd, uint8_t *buffer, size_t len, int timeout_ms);

/* SPI Interface */
int __wrap_spi_open(const char *device, int mode, int speed_hz);
int __wrap_spi_close(int fd);
int __wrap_spi_transfer(int fd, const uint8_t *tx_buf, uint8_t *rx_buf, size_t len);

/*--------------------------------------------------
 * API-Specific Mock Functions
 *--------------------------------------------------*/

/* Add any API-specific mock functions here */

#endif /* MOCK_TEMPLATE_MODULE_H */ 