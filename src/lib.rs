static mut ADVANCE_RESPONSE: [f64; 7] = [0.0; 7];
#[no_mangle]
pub extern "C" fn get_array_loc() -> *const f64 {
    unsafe {
        return ADVANCE_RESPONSE.as_ptr();
    }
}
#[no_mangle]
pub extern "C" fn advance(
    mut timeAccumulator: f64,
    solverTimeStep: f64,
    endValue: f64,
    tension: f64,
    friction: f64,
    mut position: f64,
    mut tempPosition: f64,
    mut velocity: f64,
    mut tempVelocity: f64,
    mut previousPosition: f64,
    mut previousVelocity: f64,
) {
    let mut aVelocity;
    let mut aAcceleration;
    let mut bVelocity;
    let mut bAcceleration;
    let mut cVelocity;
    let mut cAcceleration;
    let mut dVelocity;
    let mut dAcceleration;
    let mut dxdt;
    let mut dvdt;
    while timeAccumulator >= solverTimeStep {
        timeAccumulator -= solverTimeStep;
        if timeAccumulator < solverTimeStep {
            previousPosition = position;
            previousVelocity = velocity;
        }
        aVelocity = velocity;
        aAcceleration = tension * (endValue - tempPosition) - friction * velocity;

        tempPosition = position + aVelocity * solverTimeStep * 0.5;
        tempVelocity = velocity + aAcceleration * solverTimeStep * 0.5;
        bVelocity = tempVelocity;
        bAcceleration = tension * (endValue - tempPosition) - friction * tempVelocity;

        tempPosition = position + bVelocity * solverTimeStep * 0.5;
        tempVelocity = velocity + bAcceleration * solverTimeStep * 0.5;
        cVelocity = tempVelocity;
        cAcceleration = tension * (endValue - tempPosition) - friction * tempVelocity;

        tempPosition = position + cVelocity * solverTimeStep;
        tempVelocity = velocity + cAcceleration * solverTimeStep;
        dVelocity = tempVelocity;
        dAcceleration = tension * (endValue - tempPosition) - friction * tempVelocity;

        dxdt = 1.0 / 6.0 * (aVelocity + 2.0 * (bVelocity + cVelocity) + dVelocity);
        dvdt = 1.0 / 6.0 * (aAcceleration + 2.0 * (bAcceleration + cAcceleration) + dAcceleration);

        position += dxdt * solverTimeStep;
        velocity += dvdt * solverTimeStep;
        unsafe {
            ADVANCE_RESPONSE = [
                timeAccumulator,
                previousPosition,
                previousVelocity,
                tempPosition,
                tempVelocity,
                position,
                velocity,
            ];
        }
    }
}
