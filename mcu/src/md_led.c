// SPDX-License-Identifier: GPL-2.0
/**
 * @file    md_led.c
 * @author  向阳 (hinata.hoshino@foxmail.com)
 * @brief   LED
 * @version 1.0
 * @date    2023-06-26
 *
 * @copyright Copyright (c) 向阳, all rights reserved.
 *
 */
#include "md_led.h"
#include "tim.h"

extern void drv_led_init(void)
{
    MX_TIM2_Init();
    HAL_NVIC_EnableIRQ(TIM2_IRQn);
}

extern void TIM2_IRQHandler(void)
{
}
