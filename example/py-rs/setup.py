from setuptools import setup
from setuptools_rust import Binding, RustExtension

setup(
    name="hello-rust",
    version="1.0",
    rust_extensions=[RustExtension("hello_rust.hello_rust", binding=Binding.PyO3)],
    packages=["hello_rust"],
    zip_safe=False,
)
