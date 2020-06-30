import logging
from time import sleep

from faker import Faker

logging.getLogger(__name__)

logging.basicConfig(
    format='[%(asctime)s %(levelname)s] %(name)s %(message)s',
    level=logging.INFO,
    datefmt='%Y-%m-%d %H:%M:%S,111')

fake = Faker()
while True:
    logging.info(fake.name())
    sleep(7)

