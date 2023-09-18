pub struct PID {
    kp: f32,
    ki: f32,
    kd: f32,
    last_error: Option<f32>,
    integral: f32,
}

impl PID {
    pub fn new(kp: f32, ki: f32, kd: f32) -> Self {
        PID {
            kp,
            ki,
            kd,
            last_error: None,
            integral: 0.,
        }
    }

    pub fn output(&mut self, error: f32, dt: f32) -> f32 {
        self.integral += error * dt;
        let derivative: f32;
        if let Some(last_error) = self.last_error {
            derivative = (error - last_error) / dt;
        } else {
            derivative = 0.;
        }
        self.last_error = Some(error);
        self.kp * error + self.ki * self.integral + self.kd * derivative
    }
}
