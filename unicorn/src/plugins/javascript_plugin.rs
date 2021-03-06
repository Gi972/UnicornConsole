#[cfg(feature = "duktape")]
pub mod plugin {
    use std::sync::{Arc, Mutex};

    use duktape::*;
    use duktape::types::*;
    use duktape::errors::*;

    use config::Players;

    use unicorn::info::Info;
    use unicorn::noise::Noise;
    use sound::sound::Sound;

    use gfx::Screen;

    pub struct JavascriptPluginRust {
        info: Vec<Arc<Mutex<Info>>>,
        screen: Vec<Arc<Mutex<Screen>>>,
        players: Vec<Arc<Mutex<Players>>>,
        sound: Vec<Arc<Mutex<Sound>>>,
    }

    impl JavascriptPluginRust {
        pub fn new() -> JavascriptPluginRust {
            JavascriptPluginRust {
                info: Vec::new(),
                screen: Vec::new(),
                players: Vec::new(),
                sound: Vec::new(),
            }
        }

        pub fn set_info(&mut self, info: Arc<Mutex<Info>>) {
            self.info.push(info);
        }

        pub fn set_screen(&mut self, screen: Arc<Mutex<Screen>>) {
            self.screen.push(screen);
        }

        pub fn set_players(&mut self, players: Arc<Mutex<Players>>) {
            self.players.push(players);
        }


        pub fn set_sound(&mut self, sound: Arc<Mutex<Sound>>) {
            self.sound.push(sound);
        }

        pub fn cls(&self,
                   _ctx: &mut Context,
                   args: &[Value<'static>])
                   -> DuktapeResult<Value<'static>> {
            // println!("RUST CLS {:?}", args);
            let mut value: i8 = -1;

            if let Value::Number(arg) = args[0] {
                value = arg as i8;
            }

            self.screen[0].lock().unwrap().cls(value);
            Ok(Value::Number(0.))
        }

        pub fn music(&self,
                     _ctx: &mut Context,
                     args: &[Value<'static>])
                     -> DuktapeResult<Value<'static>> {
            let mut id: i32 = -1;
            let mut filename: String = "".to_string();
            let mut channel: i32 = -1;
            let mut start_position: i32 = 0;
            let mut loops: i32 = 0;

            if let Value::Number(arg) = args[0] {
                id = arg as i32;
            }

            if let Value::String(ref arg) = args[1] {
                filename = arg.to_string();
            }

            if let Value::Number(arg) = args[2] {
                loops = arg as i32;
            }

            if let Value::Number(arg) = args[3] {
                start_position = arg as i32;
            }

            if let Value::Number(arg) = args[4] {
                channel = arg as i32;
            }

            self.sound[0].lock().unwrap().music(id, filename, channel, loops, start_position);

            Ok(Value::Number(0.))
        }

        pub fn sfx(&self,
                   _ctx: &mut Context,
                   args: &[Value<'static>])
                   -> DuktapeResult<Value<'static>> {
            let mut id: i32 = -1;
            let mut filename: String = "".to_string();
            let mut channel: i32 = -1;
            let mut note: u16 = 13312;
            let mut panning: i32 = 64;
            let mut rate: i32 = 50;
            let mut loops: i32 = 0;

            if let Value::Number(arg) = args[0] {
                id = arg as i32;
            }

            if let Value::String(ref arg) = args[1] {
                filename = arg.to_string();
            }

            if let Value::Number(arg) = args[2] {
                note = arg as u16;
            }

            if let Value::Number(arg) = args[3] {
                panning = arg as i32;
            }

            if let Value::Number(arg) = args[4] {
                rate = arg as i32;
            }

            if let Value::Number(arg) = args[5] {
                loops = loops as i32;
            }

            if let Value::Number(arg) = args[6] {
                channel = channel as i32;
            }

            self.sound[0].lock().unwrap().sfx(id, filename, channel, note, panning, rate, loops);

            Ok(Value::Number(0.))
        }

        pub fn btnp(&self,
                    _ctx: &mut Context,
                    args: &[Value<'static>])
                    -> DuktapeResult<Value<'static>> {
            // info!("RUST BTNP {:?}", args);
            let mut p: u8 = 0;
            let mut x: u8 = 0;

            if let Value::Number(arg) = args[0] {
                x = arg as u8;
            }

            if let Value::Number(arg) = args[1] {
                p = arg as u8;
            }

            let value = self.players[0].lock().unwrap().btnp(p, x);

            Ok(Value::Bool(value))
        }

        pub fn unicorn_time(&self,
                            _ctx: &mut Context,
                            args: &[Value<'static>])
                            -> DuktapeResult<Value<'static>> {
            // info!("RUST TIME {:?}", args);

            Ok(Value::Number(self.info[0].lock().unwrap().time() as f64))
        }

        pub fn print(&self,
                     _ctx: &mut Context,
                     args: &[Value<'static>])
                     -> DuktapeResult<Value<'static>> {
            let mut text: String = "".to_string();
            let mut x: i32 = 0;
            let mut y: i32 = 0;
            let mut color: i32 = -1;

            if args.len() < 3 {
                return Ok(Value::Number(-1.));
            }

            // info!("RUST PRINT {:?}", args);

            if let Value::String(ref n) = args[0] {
                text = n.to_string();
            }

            if let Value::Number(n) = args[1] {
                x = n as i32;
            }

            if let Value::Number(n) = args[2] {
                y = n as i32;
            }

            match args[3] {
                Value::Number(n) => color = n as i32,
                _ => (),
            }

            self.screen[0].lock().unwrap().print(text, x, y, color);

            Ok(Value::Number(0.))
        }

        pub fn pset(&self,
                    _ctx: &mut Context,
                    args: &[Value<'static>])
                    -> DuktapeResult<Value<'static>> {
            let mut x: i32 = 0;
            let mut y: i32 = 0;
            let mut color: i32 = -1;

            if args.len() < 2 {
                return Ok(Value::Number(-1.));
            }

            // println!("RUST PSET {:?}", args);

            if let Value::Number(n) = args[0] {
                x = n as i32;
            }

            if let Value::Number(n) = args[1] {
                y = n as i32;
            }

            match args[2] {
                Value::Number(n) => color = n as i32,
                _ => (),
            }

            self.screen[0].lock().unwrap().pset(x, y, color);

            Ok(Value::Number(0.))
        }

        pub fn spr(&self,
                   _ctx: &mut Context,
                   args: &[Value<'static>])
                   -> DuktapeResult<Value<'static>> {
            let mut n: u32 = 0;
            let mut x: i32 = 0;
            let mut y: i32 = 0;
            let mut w: u32 = 1;
            let mut h: u32 = 1;
            let mut flip_x: bool = false;
            let mut flip_y: bool = false;

            if args.len() < 3 {
                return Ok(Value::Number(-1.));
            }

            // info!("RUST SPR {:?}", args);

            if let Value::Number(arg) = args[0] {
                n = arg as u32;
            }

            if let Value::Number(arg) = args[1] {
                x = arg as i32;
            }

            if let Value::Number(arg) = args[2] {
                y = arg as i32;
            }

            if let Value::Number(arg) = args[3] {
                w = arg as u32;
            }

            if let Value::Number(arg) = args[4] {
                h = arg as u32;
            }

            if let Value::Bool(arg) = args[5] {
                flip_x = arg as bool;
            }

            if let Value::Bool(arg) = args[6] {
                flip_y = arg as bool;
            }


            self.screen[0].lock().unwrap().spr(n, x, y, w, h, flip_x, flip_y);

            Ok(Value::Number(0.))
        }

        pub fn sspr(&self,
                    _ctx: &mut Context,
                    args: &[Value<'static>])
                    -> DuktapeResult<Value<'static>> {
            let mut sx: u32 = 0;
            let mut sy: u32 = 0;
            let mut sw: u32 = 0;
            let mut sh: u32 = 0;
            let mut dx: i32 = 0;
            let mut dy: i32 = 0;
            let mut dw: u32 = 0;
            let mut dh: u32 = 0;

            let mut flip_x: bool = false;
            let mut flip_y: bool = false;

            // info!("RUST SSPR {:?}", args);

            if let Value::Number(arg) = args[0] {
                sx = arg as u32;
            }

            if let Value::Number(arg) = args[1] {
                sy = arg as u32;
            }

            if let Value::Number(arg) = args[2] {
                sw = arg as u32;
            }

            if let Value::Number(arg) = args[3] {
                sh = arg as u32;
            }

            if let Value::Number(arg) = args[4] {
                dx = arg as i32;
            }

            if let Value::Number(arg) = args[5] {
                dy = arg as i32;
            }

            if let Value::Number(arg) = args[6] {
                dw = arg as u32;
            } else {
                dw = sw as u32;
            }

            if let Value::Number(arg) = args[7] {
                dh = arg as u32;
            } else {
                dh = sh as u32;
            }

            if let Value::Bool(arg) = args[8] {
                flip_x = arg as bool;
            }

            if let Value::Bool(arg) = args[9] {
                flip_y = arg as bool;
            }


            self.screen[0].lock().unwrap().sspr(sx, sy, sw, sh, dx, dy, dw, dh, flip_x, flip_y);

            Ok(Value::Number(0.))
        }

        pub fn sspr2(&self,
                     _ctx: &mut Context,
                     args: &[Value<'static>])
                     -> DuktapeResult<Value<'static>> {
            let mut sx: u32 = 0;
            let mut sy: u32 = 0;
            let mut sw: u32 = 0;
            let mut sh: u32 = 0;
            let mut dx: i32 = 0;
            let mut dy: i32 = 0;
            let mut angle: f64 = 0.0;
            let mut zoom: f64 = 1.0;

            let mut flip_x: bool = false;
            let mut flip_y: bool = false;

            if let Value::Number(arg) = args[0] {
                sx = arg as u32;
            }

            if let Value::Number(arg) = args[1] {
                sy = arg as u32;
            }

            if let Value::Number(arg) = args[2] {
                sw = arg as u32;
            }

            if let Value::Number(arg) = args[3] {
                sh = arg as u32;
            }

            if let Value::Number(arg) = args[4] {
                dx = arg as i32;
            }

            if let Value::Number(arg) = args[5] {
                dy = arg as i32;
            }

            if let Value::Number(arg) = args[6] {
                angle = arg as f64;
            }

            if let Value::Number(arg) = args[7] {
                zoom = arg as f64;
            }

            if let Value::Bool(arg) = args[8] {
                flip_x = arg as bool;
            }

            if let Value::Bool(arg) = args[9] {
                flip_y = arg as bool;
            }

            self.screen[0]
                .lock()
                .unwrap()
                .sspr2(sx, sy, sw, sh, dx, dy, angle, zoom, flip_x, flip_y);

            Ok(Value::Number(0.))
        }

        pub fn pal(&self,
                   _ctx: &mut Context,
                   args: &[Value<'static>])
                   -> DuktapeResult<Value<'static>> {
            // println!("RUST CLS {:?}", args);
            let mut c0: i32 = -1;
            let mut c1: i32 = -1;

            if let Value::Number(arg) = args[0] {
                c0 = arg as i32;
            }

            if let Value::Number(arg) = args[1] {
                c1 = arg as i32;
            }

            self.screen[0].lock().unwrap().pal(c0, c1);

            Ok(Value::Number(0.))
        }

        pub fn circ(&self,
                    _ctx: &mut Context,
                    args: &[Value<'static>])
                    -> DuktapeResult<Value<'static>> {
            let mut x: i32 = -1;
            let mut y: i32 = -1;
            let mut r: i32 = -1;
            let mut color: i32 = -1;

            if let Value::Number(arg) = args[0] {
                x = arg as i32;
            }

            if let Value::Number(arg) = args[1] {
                y = arg as i32;
            }

            if let Value::Number(arg) = args[2] {
                r = arg as i32;
            }

            if let Value::Number(arg) = args[3] {
                color = color as i32;
            }

            self.screen[0].lock().unwrap().circ(x, y, r, color);

            Ok(Value::Number(0.))
        }

        pub fn circfill(&self,
                        _ctx: &mut Context,
                        args: &[Value<'static>])
                        -> DuktapeResult<Value<'static>> {
            let mut x: i32 = -1;
            let mut y: i32 = -1;
            let mut r: i32 = -1;
            let mut color: i32 = -1;

            if let Value::Number(arg) = args[0] {
                x = arg as i32;
            }

            if let Value::Number(arg) = args[1] {
                y = arg as i32;
            }

            if let Value::Number(arg) = args[2] {
                r = arg as i32;
            }

            if let Value::Number(arg) = args[3] {
                color = arg as i32;
            }

            self.screen[0].lock().unwrap().circfill(x, y, r, color);

            Ok(Value::Number(0.))
        }

        pub fn line(&self,
                    _ctx: &mut Context,
                    args: &[Value<'static>])
                    -> DuktapeResult<Value<'static>> {
            let mut x1: i32 = -1;
            let mut y1: i32 = -1;
            let mut x2: i32 = -1;
            let mut y2: i32 = -1;
            let mut color: i32 = -1;

            if let Value::Number(arg) = args[0] {
                x1 = arg as i32;
            }

            if let Value::Number(arg) = args[1] {
                y1 = arg as i32;
            }

            if let Value::Number(arg) = args[2] {
                x2 = arg as i32;
            }

            if let Value::Number(arg) = args[3] {
                y2 = arg as i32;
            }

            if let Value::Number(arg) = args[4] {
                color = arg as i32;
            }

            self.screen[0].lock().unwrap().line(x1, y1, x2, y2, color);

            Ok(Value::Number(0.))
        }
    }

    impl Foo for JavascriptPluginRust {
        fn dispatch(&mut self,
                    _ctx: &mut Context,
                    args: &[Value<'static>])
                    -> DuktapeResult<Value<'static>> {
            //  println!("I'm here {:?}", args);
            //  println!("CONTEXT {:?}", _ctx.dump_context());

            if let Value::Number(function_idx) = args[0] {
                let args = &args[1..args.len()];

                let function_idx = function_idx as i32;

                match function_idx {
                    0x1 => return self.pset(_ctx, args),
                    0x2 => return self.cls(_ctx, args),
                    0x3 => return self.unicorn_time(_ctx, args),
                    0x4 => return self.print(_ctx, args),
                    0x5 => return self.spr(_ctx, args),
                    0x6 => return self.btnp(_ctx, args),
                    0x7 => return self.sspr(_ctx, args),
                    0x8 => return self.pal(_ctx, args),
                    0x9 => return self.sfx(_ctx, args),
                    0x10 => return self.music(_ctx, args),
                    0x11 => return self.circ(_ctx, args),
                    0x12 => return self.circfill(_ctx, args),
                    0x13 => return self.line(_ctx, args),
                    0x14 => return self.sspr2(_ctx, args),

                    _ => (),
                }
            }

            Ok(Value::Number(0.))
        }
    }

    pub struct JavascriptPlugin {
        ctx: Context,
        javascript: Arc<Mutex<JavascriptPluginRust>>,
        loaded_code: bool,
    }

    impl JavascriptPlugin {
        pub fn new() -> JavascriptPlugin {
            JavascriptPlugin {
                ctx: Context::new(),
                javascript: Arc::new(Mutex::new(JavascriptPluginRust::new())),
                loaded_code: false,
            }
        }

        pub fn load(&mut self,
                    players: Arc<Mutex<Players>>,
                    info: Arc<Mutex<Info>>,
                    screen: Arc<Mutex<Screen>>,
                    noise: Arc<Mutex<Noise>>,
                    sound: Arc<Mutex<Sound>>) {
            info!("[PLUGIN][JAVASCRIPT] Init plugin");
            self.javascript.lock().unwrap().set_info(info.clone());
            self.javascript.lock().unwrap().set_screen(screen.clone());
            self.javascript.lock().unwrap().set_players(players.clone());
            self.javascript.lock().unwrap().set_sound(sound.clone());

            self.ctx.register(0x1, "pset", self.javascript.clone(), Some(3));
            self.ctx.register(0x2, "cls", self.javascript.clone(), Some(1));
            self.ctx.register(0x3, "unicorn_time", self.javascript.clone(), Some(0));
            self.ctx.register(0x4, "print", self.javascript.clone(), Some(4));
            self.ctx.register(0x5, "spr", self.javascript.clone(), Some(7));
            self.ctx.register(0x6, "btnp", self.javascript.clone(), Some(2));
            self.ctx.register(0x7, "sspr", self.javascript.clone(), Some(10));
            self.ctx.register(0x8, "pal", self.javascript.clone(), Some(2));
            self.ctx.register(0x9, "sfx", self.javascript.clone(), Some(7));
            self.ctx.register(0x10, "music", self.javascript.clone(), Some(5));
            self.ctx.register(0x11, "circ", self.javascript.clone(), Some(4));
            self.ctx.register(0x12, "circfill", self.javascript.clone(), Some(4));
            self.ctx.register(0x13, "line", self.javascript.clone(), Some(5));
            self.ctx.register(0x14, "sspr2", self.javascript.clone(), Some(10));

        }

        pub fn init(&mut self) {
            if !self.loaded_code {
                return;
            }

            match self.ctx.eval("_init();") {
                Result::Ok(_) => (),
                Result::Err(err) => warn!("Error during the init function {:?}", err),
            }
        }

        pub fn draw(&mut self) -> bool {
            if !self.loaded_code {
                return false;
            }

            match self.ctx.eval("_draw();") {
                Result::Ok(_) => (),
                Result::Err(err) => warn!("Error during the draw function {:?}", err),
            }

            true
        }

        pub fn update(&mut self) -> bool {
            if !self.loaded_code {
                return false;
            }
            match self.ctx.eval("_update();") {
                Result::Ok(_) => (),
                Result::Err(err) => warn!("Error during the update function {:?}", err),
            }

            true
        }

        pub fn load_code(&mut self, data: String) -> bool {
            info!("[PLUGIN][JAVASCRIPT] LOAD CODE");

            match self.ctx.eval(&data) {
                Result::Ok(_) => self.loaded_code = true,
                Result::Err(err) => {
                    self.loaded_code = false;
                    warn!("Error to load the code {:?}", err);
                }
            }

            self.loaded_code
        }
    }
}

#[cfg(not(feature = "duktape"))]
pub mod plugin {
    use std::sync::{Arc, Mutex};

    use config::Players;

    use unicorn::noise::Noise;
    use unicorn::info::Info;
    use sound::sound::Sound;

    use gfx::Screen;


    pub struct JavascriptPlugin {}

    impl JavascriptPlugin {
        pub fn new() -> JavascriptPlugin {
            JavascriptPlugin {}
        }

        // Keep the compatibility
        pub fn load(&mut self,
                    _players: Arc<Mutex<Players>>,
                    _info: Arc<Mutex<Info>>,
                    _screen: Arc<Mutex<Screen>>,
                    _noise: Arc<Mutex<Noise>>,
                    _sound: Arc<Mutex<Sound>>) {
            panic!("Javascript plugin disabled");
        }
        pub fn load_code(&mut self, _data: String) -> bool {
            false
        }
        pub fn init(&mut self) {}
        pub fn draw(&mut self) -> bool {
            false
        }
        pub fn update(&mut self) -> bool {
            false
        }
    }
}
