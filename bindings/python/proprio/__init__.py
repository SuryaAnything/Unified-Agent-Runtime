from .client import ProprioClient

def connect(app_id):
    return ProprioClient(app_id)