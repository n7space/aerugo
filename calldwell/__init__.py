import logging


def init_default_logger():
    logging.basicConfig(
        level=logging.INFO, format="[%(asctime)s] <%(levelname)s|%(name)s>: %(message)s"
    )
