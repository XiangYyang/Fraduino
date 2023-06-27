// SPDX-License-Identifier: GPL-2.0
/**
 * @file    ma_main.c
 * @author  向阳 (hinata.hoshino@foxmail.com)
 * @brief   主程序
 * @version 1.0
 * @date    2023-06-26
 *
 * @copyright Copyright (c) 向阳, all rights reserved.
 *
 */
#include "ma_main.h"
#include "gpio.h"
#include "main.h"
#include "md_led.h"

extern void SystemClock_Config(void);
extern void PeriphCommonClock_Config(void);

int main(void)
{
    HAL_Init();
    SystemClock_Config();
    PeriphCommonClock_Config();
    MX_GPIO_Init();
    // 初始化外设
    drv_led_init();
    while (1) {
        // nop
    }
}
