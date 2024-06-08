fn main() {
    println!("cargo:rustc-link-search=native=C:\\Users\\fabio\\Desktop\\uni\\MachineLearning\\squaro_py_in_ru\\squaro\\.venv\\Lib");
    println!("cargo:rustc-link-lib=python312"); // Adjust to match the exact version
    println!("cargo:include=C:\\Users\\fabio\\Desktop\\uni\\MachineLearning\\squaro_py_in_ru\\squaro\\.venv\\Include");
}
