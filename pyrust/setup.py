from setuptools import setup
from setuptools_rust import Binding, RustExtension

setup(
    name="string-sum",
    version="1.0",
    rust_extensions=[RustExtension("string_sum.string_sum", binding=Binding.PyO3)],
    packages=["string_sum"],
    include_package_data=True,
    # rust extensions are not zip safe, just like C-extensions.
    zip_safe=False,
)

