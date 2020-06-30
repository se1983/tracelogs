FROM python:3.7-buster

RUN apt-get update && apt-get -y install python3-pip
RUN pip3 install Faker

COPY run.py /run.py
CMD && chmod +x /run.py
CMD python3 /run.py
