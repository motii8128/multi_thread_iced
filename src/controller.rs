extern crate hidapi;
use crate::interface::{Buttons, JoyStick, DualShock4, Dpad};
use hidapi::{HidApi, HidDevice, HidError};

pub const BLE:u8 = 50;
pub const SERIAL:u8 = 100;

pub struct DualShock4Driver
{
    device:HidDevice,
    mode:u8,
}


impl DualShock4Driver {
    pub fn new(mode_:u8)->Result<DualShock4Driver, HidError>
    {
        let api = HidApi::new().unwrap();

        match api.open(1356, 2508)
        {
            Ok(dev)=>{
                let ds = DualShock4Driver
                {
                    device:dev,
                    mode:mode_,
                };
                Ok(ds)
            }
            Err(e)=>{
                Err(e)
            }
        }

    }
    pub fn task(&mut self)->Result<DualShock4, HidError>
    {
            let mut buf = [0_u8;256];

            match self.device.read(&mut buf) {
                Ok(size)=>{
                    let get_data = &buf[..size];
                    // println!("{:?}", get_data);

                    let (j, btn, d) = convert(get_data, self.mode);

                    Ok(DualShock4 { sticks: j, btns: btn, dpad: d })
                }
                Err(e)=>{
                    Err(e)
                }
            }
    }
    
}


fn convert(buf:&[u8], mode:u8)->(JoyStick, Buttons, Dpad)
{
    if mode == BLE
    {
        let l_x = map(buf[3], 0.0, 255.0, -1.0, 1.0);
        let l_y = map(buf[4], 0.0, 255.0, 1.0, -1.0);
        let r_x = map(buf[5], 0.0, 255.0, -1.0, 1.0);
        let r_y = map(buf[6], 0.0, 255.0, 1.0, -1.0);
        let joy = JoyStick{left_x:l_x,left_y:l_y,right_x:r_x,right_y:r_y};
        let mut btns = Buttons{
            circle:false,
            cross:false,
            triangle:false,
            cube:false,
            r1:false,
            r2:false,
            l1:false,
            l2:false,
            left_push:false,
            right_push:false,
        };

        let mut dpad = Dpad{
            up_key:false,
            down_key:false,
            right_key:false,
            left_key:false
        };

        match buf[7] {
            0=>dpad.up_key = true,
            2=>dpad.right_key = true,
            4=>dpad.down_key = true,
            6=>dpad.left_key = true,
            24=>btns.cube = true,
            40=>btns.cross = true,
            72=>btns.circle = true,
            136=>btns.triangle = true,
            8=>(),
            _=>()
        }

        match buf[8] {
            1=>btns.l1 = true,
            2=>btns.r1 = true,
            4=>btns.l2 = true,
            8=>btns.r2 = true,
            64=>btns.left_push = true,
            128=>btns.right_push = true,
            _=>(),
        }
        (joy, btns, dpad)
    }
    else if mode == SERIAL
    {
        let l_x = map(buf[1], 0.0, 255.0, -1.0, 1.0);
        let l_y = map(buf[2], 0.0, 255.0, 1.0, -1.0);
        let r_x = map(buf[3], 0.0, 255.0, -1.0, 1.0);
        let r_y = map(buf[4], 0.0, 255.0, 1.0, -1.0);
        let joy = JoyStick{left_x:l_x,left_y:l_y,right_x:r_x,right_y:r_y};
        let mut btns = Buttons{
            circle:false,
            cross:false,
            triangle:false,
            cube:false,
            r1:false,
            r2:false,
            l1:false,
            l2:false,
            left_push:false,
            right_push:false,
        };

        let mut dpad = Dpad{
            up_key:false,
            down_key:false,
            right_key:false,
            left_key:false
        };

        match buf[5] {
            0=>dpad.up_key = true,
            1=>{dpad.up_key = true;dpad.right_key = true},
            2=>dpad.right_key = true,
            3=>{dpad.right_key = true;dpad.down_key = true},
            4=>dpad.down_key = true,
            5=>{dpad.left_key=true;dpad.down_key=true},
            6=>dpad.left_key = true,
            7=>{dpad.left_key=true;dpad.up_key=true},
            24=>btns.cube = true,
            40=>btns.cross = true,
            56=>{btns.cube=true;btns.cross=true},
            72=>btns.circle = true,
            88=>{btns.circle = true;btns.cube=true},
            104=>{btns.circle=true;btns.cross=true},
            136=>btns.triangle = true,
            152=>{btns.cube=true;btns.triangle=true},
            168=>{btns.triangle=true;btns.cross=true},
            200=>{btns.triangle=true;btns.circle=true},
            8=>(),
            _=>()
        }

        match buf[6] {
            1=>btns.l1 = true,
            2=>btns.r1 = true,
            4=>btns.l2 = true,
            8=>btns.r2 = true,
            64=>btns.left_push = true,
            128=>btns.right_push = true,
            _=>(),
        }
        (joy, btns, dpad)
    }
    else {

        let joy = JoyStick{left_x:0.0, left_y:0.0, right_x:0.0, right_y:0.0};
        let btns = Buttons{
            circle:false,
            cross:false,
            triangle:false,
            cube:false,
            r1:false,
            r2:false,
            l1:false,
            l2:false,
            left_push:false,
            right_push:false,
        };

        let dpad = Dpad{
            up_key:false,
            down_key:false,
            right_key:false,
            left_key:false
        };

        (joy, btns, dpad)
    }
}   

fn map(value:u8,in_min:f32, in_max:f32, out_min:f32, out_max:f32)->f32
{
    let mut result = (value as f32 - in_min) * (out_max - out_min) / (in_max - in_min) + out_min;

    if result.abs() < 0.07
    {
        result = 0.0;
    }

    result
}