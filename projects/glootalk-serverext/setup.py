import os
from setuptools import setup
from jupyter_packaging import create_cmdclass

VERSION = '0.0.1'

setup_args = dict(
    name = 'jupygloo',
    version = VERSION,
    description = 'Jupyter Server Extension for glootalk',
    python_requires = '>=3.6',
    install_requires = [
        'jupyter_server',
        'glootalk',
    ],
    entry_points = {
        'console_scripts': [
            'jupygloo = jupygloo_server.app:main'
            ]
    },
    include_package_data=True,
)

setup(**setup_args)
