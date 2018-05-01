/*
 * Copyright (C) 2018 Kubos Corporation
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use mai400::*;
use super::*;

static RAW_READ: [u8; 238] = [
    0x90,
    0xEB,
    0x3,
    0x93,
    0x3C,
    0x74,
    0x47,
    0x0,
    0x2,
    0x0,
    0x0,
    0x0,
    0x0,
    0x0,
    0x44,
    0x1,
    0x0,
    0x0,
    0x4,
    0x0,
    0x1,
    0x0,
    0x0,
    0x0,
    0x0,
    0x0,
    0x4,
    0x0,
    0x1,
    0x1,
    0x80,
    0x1,
    0x80,
    0x1,
    0x80,
    0xA7,
    0xFA,
    0x69,
    0x0,
    0xEF,
    0xFC,
    0x7A,
    0xFB,
    0xE9,
    0xB5,
    0x37,
    0xC0,
    0xA,
    0x34,
    0x78,
    0x27,
    0x86,
    0xB5,
    0x0,
    0x0,
    0x0,
    0x0,
    0x0,
    0x0,
    0x0,
    0x0,
    0x0,
    0x0,
    0x0,
    0x0,
    0x0,
    0x0,
    0x0,
    0x0,
    0x0,
    0x0,
    0x0,
    0x0,
    0x0,
    0x0,
    0x0,
    0x0,
    0x0,
    0x0,
    0x0,
    0x1B,
    0x2F,
    0xDD,
    0x3D,
    0x8C,
    0xB7,
    0x53,
    0xBC,
    0xF9,
    0xB3,
    0xCC,
    0x3D,
    0x7F,
    0xF1,
    0x76,
    0x0,
    0x0,
    0x0,
    0x0,
    0x0,
    0x0,
    0xFF,
    0x7F,
    0x0,
    0x0,
    0x0,
    0x0,
    0x0,
    0x0,
    0xFF,
    0x7F,
    0x0,
    0x0,
    0x0,
    0x0,
    0x0,
    0x0,
    0x0,
    0x0,
    0x0,
    0x0,
    0xFF,
    0x7F,
    0x0,
    0x0,
    0x0,
    0x0,
    0x0,
    0x0,
    0x0,
    0x0,
    0x0,
    0x0,
    0x0,
    0x0,
    0xBF,
    0xE9,
    0x54,
    0xBE,
    0x34,
    0x56,
    0xAD,
    0x40,
    0x2A,
    0x56,
    0xAD,
    0x40,
    0x19,
    0x7C,
    0x19,
    0x7C,
    0x19,
    0x7C,
    0xE,
    0x80,
    0x8D,
    0xFD,
    0x8D,
    0xFD,
    0xBD,
    0x26,
    0x91,
    0xEA,
    0x34,
    0x0,
    0xA6,
    0x1,
    0x0,
    0x0,
    0x0,
    0x0,
    0x0,
    0x0,
    0x0,
    0x0,
    0x0,
    0x0,
    0x0,
    0x0,
    0x0,
    0x0,
    0x0,
    0x0,
    0x0,
    0x0,
    0x0,
    0x0,
    0x0,
    0x0,
    0x0,
    0x0,
    0x0,
    0x0,
    0x0,
    0x0,
    0x0,
    0x0,
    0x0,
    0x0,
    0x0,
    0x0,
    0x0,
    0x0,
    0x0,
    0x0,
    0x0,
    0x0,
    0x8,
    0x8,
    0x0,
    0x0,
    0x8,
    0x8,
    0x0,
    0x0,
    0x2A,
    0xA1,
    0x91,
    0xEA,
    0x11,
    0x0,
    0xC8,
    0x0,
    0x1,
    0x0,
    0xFB,
    0xFF,
    0x10,
    0x1,
    0x26,
    0x0,
    0x1D,
    0x0,
    0x16,
    0x0,
    0x13,
    0x82,
    0x93,
];

#[test]
fn get_message_bad() {
    let mock = mock_new!();

    let mai = MAI400 { conn: Connection { stream: Box::new(mock) } };

    assert_eq!(mai.get_message().unwrap_err(), MAIError::GenericError);
}

#[test]
fn get_message_good_stdtelem() {
    let mock = mock_new!();

    let expected = Some(StandardTelemetry {
        tlm_counter: 3,
        gps_time: 1198800019,
        time_subsec: 0,
        cmd_valid_cntr: 2,
        cmd_invalid_cntr: 0,
        cmd_invalid_chksum_cntr: 0,
        last_command: 0x44,
        acs_mode: 1,
        css: [0, 4, 1, 0, 0, 4],
        eclipse_flag: 1,
        sun_vec_b: [-32767, -32767, -32767],
        i_b_field_meas: [-1369, 105, -785],
        bd: [-0.0000017433042, 0.00000012922179, -0.0000009995265],
        rws_speed_cmd: [0, 0, 0],
        rws_speed_tach: [0, 0, 0],
        rwa_torque_cmd: [0.0, 0.0, 0.0],
        gc_rwa_torque_cmd: [0, 0, 0],
        torque_coil_cmd: [0.108, -0.012922179, 0.099952646],
        gc_torque_coil_cmd: [127, -15, 118],
        qbo_cmd: [0, 0, 0, 32767],
        qbo_hat: [0, 0, 0, 32767],
        angle_to_go: 0.0,
        q_error: [0, 0, 0, 32767],
        omega_b: [0.0, 0.0, 0.0],
        rotating_variable_a: 0xBE54E9BF,
        rotating_variable_b: 0x40AD5634,
        rotating_variable_c: 0x40AD562A,
        nb: [31769, 31769, 31769],
        neci: [-32754, -627, -627],
    });

    mock.read.return_value(Ok(RAW_READ.to_vec()));

    let mai = MAI400 { conn: Connection { stream: Box::new(mock) } };

    let (result, _, _) = mai.get_message().unwrap();

    assert_eq!(result, expected);
}

#[test]
fn get_message_good_rawimu() {
    let mock = mock_new!();

    let expected = Some(RawIMU {
        accel: [1, -5, 272],
        gyro: [38, 29, 22],
        gyro_temp: 19,
    });

    mock.read.return_value(Ok(RAW_READ.to_vec()));

    let mai = MAI400 { conn: Connection { stream: Box::new(mock) } };

    let (_, result, _) = mai.get_message().unwrap();

    assert_eq!(result, expected);
}

#[test]
fn get_message_good_irehs() {
    let mock = mock_new!();

    let expected = Some(IREHSTelemetry {
        thermopiles_a: [0, 0, 0, 0],
        thermopiles_b: [0, 0, 0, 0],
        temp_a: [0, 0, 0, 0],
        temp_b: [0, 0, 0, 0],
        dip_angle_a: 0,
        dip_angle_b: 0,
        solution_degraded: [
            ThermopileFlags::NO_COMM,
            ThermopileFlags::NO_COMM,
            ThermopileFlags::empty(),
            ThermopileFlags::empty(),
            ThermopileFlags::NO_COMM,
            ThermopileFlags::NO_COMM,
            ThermopileFlags::empty(),
            ThermopileFlags::empty(),
        ],
    });

    mock.read.return_value(Ok(RAW_READ.to_vec()));

    let mai = MAI400 { conn: Connection { stream: Box::new(mock) } };

    let (_, _, result) = mai.get_message().unwrap();

    assert_eq!(result, expected);
}