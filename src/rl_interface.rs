use cpython::{Python, PyObject, PyResult, PythonObject, PyDict, PyTuple};

pub struct RLInterface {
    rl_module: PyObject,
}

impl RLInterface {
    pub fn new(py: Python) -> PyResult<Self> {
        let rl_module = py.import("rl")?;
        Ok(RLInterface { rl_module: rl_module.into_object() })
    }

    pub fn compute_action(&self, py: Python, state: &str) -> PyResult<String> {
        let locals = PyDict::new(py);
        locals.set_item(py, "rl_module", &self.rl_module)?;
        locals.set_item(py, "state", state)?;

        let result = py.eval("rl_module.compute_action(state)", None, Some(&locals))?;
        result.extract(py)
    }

    pub fn learn(&self, py: Python, state: &str, action: &str, reward: f32, next_state: &str) -> PyResult<()> {
        let locals = PyDict::new(py);
        locals.set_item(py, "rl_module", &self.rl_module)?;
        locals.set_item(py, "state", state)?;
        locals.set_item(py, "action", action)?;
        locals.set_item(py, "reward", reward)?;
        locals.set_item(py, "next_state", next_state)?;

        py.run("rl_module.learn(state, action, reward, next_state)", None, Some(&locals))?;
        Ok(())
    }

    pub fn decay_epsilon(&self, py: Python) -> PyResult<()> {
        let locals = PyDict::new(py);
        locals.set_item(py, "rl_module", &self.rl_module)?;
        
        py.run("rl_module.decay_epsilon()", None, Some(&locals))?;
        Ok(())
    }
}
