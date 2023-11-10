use jni::{JavaVM, objects::{JObject, JValue}, AttachGuard};

pub type Error = Box<dyn std::error::Error>;

type Result<T> = std::result::Result<T, Error>;

pub struct VibratorService<'a> {
    env: AttachGuard<'a>,
    object: JObject<'a>,
}

impl VibratorService<'_> {
    pub fn get() -> Result<Self> {
        let ctx = ndk_context::android_context();
        let vm = unsafe { JavaVM::from_raw(ctx.vm().cast()) }?;
        let mut env = vm.attach_current_thread()?;

        // Fetch the PowerManager system service.
        let power_manager_service_id = env.new_string("power")?;
        let power_manager = catch_exceptions(&mut env, |env| {
            env.call_method(
                unsafe { JObject::from_raw(ctx.context().cast()) },
                "getSystemService",
                "(Ljava/lang/String;)Ljava/lang/Object;",
                &[JValue::from(&power_manager_service_id)],
            )?
            .l()
        })?;

        todo!()
    }
}

pub struct VibrationEffect {}

pub struct CombinedVibration<'a> {
    object: JObject<'a>,
}

impl CombinedVibration<'_> {
    pub fn create_parallel(effect: VibrationEffect) {

    }
}

/// Helper for handling Java exceptions thrown when entering Java code that turns
/// thrown exceptions into formatted Rust errors.
#[inline]
fn catch_exceptions<'a, T, F>(env: &mut jni::JNIEnv<'a>, f: F) -> Result<T>
where
    F: FnOnce(&mut jni::JNIEnv<'a>) -> jni::errors::Result<T>,
{
    match f(env) {
        Ok(value) => Ok(value),
        Err(e @ jni::errors::Error::JavaException) => Err({
            if let Ok(exception) = env.exception_occurred() {
                let _ = env.exception_clear();

                env.call_method(exception, "getMessage", "()Ljava/lang/String;", &[])
                    .and_then(|value| value.l())
                    .and_then(|message| {
                        env.get_string(&message.into())
                            .map(|s| s.to_string_lossy().into_owned())
                    })
                    .map(|message| message.into())
                    .unwrap_or_else(|_| e.into())
            } else {
                e.into()
            }
        }),
        Err(e) => Err(e.into()),
    }
}
