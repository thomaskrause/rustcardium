use super::*;

use arrayvec::ArrayVec;

pub const MAX_BUFFER_SIZE: usize = 1024;

/// Configuration for a BHI160 sensor.
///
///  This struct is used when enabling a sensor using `epic_bhi160_enable_sensor()`.
pub struct SensorConfig {
    pub sample_buffer_len: usize,
    pub sample_rate: u16,
    pub dynamic_range: u16,
}

#[derive(Clone)]
pub enum SensorType {
    Accelerometer = 0,
    Orientation = 1,
    Gyroscope = 2,
}

pub struct Sensor {
    sd: Option<i32>,
    sensor_type: SensorType,
    cfg: SensorConfig,
}

pub struct DataVector {
    pub x: i16,
    pub y: i16,
    pub z: i16,
    pub status: u8,
}

impl Sensor {
    pub fn enable(sensor_type: SensorType, cfg: SensorConfig) -> Sensor {
        let mut raw_cfg = sys::bhi160_sensor_config {
            sample_buffer_len: cfg.sample_buffer_len,
            sample_rate: cfg.sample_rate,
            dynamic_range: cfg.dynamic_range,
            _padding: [0, 0, 0, 0, 0, 0, 0, 0],
        };
        let sd =
            unsafe { sys::epic_bhi160_enable_sensor(sensor_type.clone() as u32, &mut raw_cfg) };
        Sensor {
            sd: Some(sd),
            sensor_type,
            cfg,
        }
    }

    pub fn read(&self) -> ArrayVec<[DataVector; MAX_BUFFER_SIZE]> {
        let mut result = ArrayVec::<[DataVector; MAX_BUFFER_SIZE]>::new();
        match self.sd {
            Some(sd) => {
                let default_buffer_value = sys::bhi160_data_vector {
                    x: 0,
                    y: 0,
                    z: 0,
                    status: 0,
                    data_type: 0,
                };
                let mut buffer: [sys::bhi160_data_vector; MAX_BUFFER_SIZE] =
                    [default_buffer_value; MAX_BUFFER_SIZE];

                let n: usize = unsafe {
                    sys::epic_stream_read(
                        sd,
                        &mut buffer as *mut _ as *mut core::ffi::c_void,
                        self.cfg.sample_buffer_len,
                    ) as usize
                };
                for i in 0..n {
                    result.push(DataVector {
                        x: buffer[i].x,
                        y: buffer[i].y,
                        z: buffer[i].z,
                        status: buffer[i].status,
                    });
                }
            }
            None => {}
        }
        result
    }

    pub fn disable(&mut self) {
        unsafe { sys::epic_bhi160_disable_sensor(self.sensor_type.clone() as u32) };
        self.sd = None;
    }
}

impl Drop for Sensor {
    fn drop(&mut self) {
        self.disable();
    }
}
