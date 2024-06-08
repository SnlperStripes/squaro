use cpython::{Python, PyResult, PyObject, PyModule, PythonObject, ObjectProtocol};

pub struct RLInterface {
    rl_module: PyObject,
}

impl RLInterface {
    pub fn new(py: Python) -> PyResult<Self> {
        let rl_module = PyModule::import(py, "rl")?.into_object();
        Ok(RLInterface { rl_module })
    }

    pub fn compute_action(&self, py: Python, state: &str) -> PyResult<String> {
        let compute_action = self.rl_module.getattr(py, "compute_action")?;
        let action: String = compute_action.call(py, (state,), None)?.extract(py)?;
        Ok(action)
    }

    pub fn learn(&self, py: Python, state: &str, action: &str, reward: f32, next_state: &str) -> PyResult<()> {
        let learn = self.rl_module.getattr(py, "learn")?;
        learn.call(py, (state, action, reward, next_state), None)?;
        Ok(())
    }
}
