from flask import Flask, request
import rpkv

api = Flask(__name__)

@api.route('/store', methods=['GET'])
def get():
  return rpkv.get_path()

@api.route('/store/<key>', methods=['GET'])
def get_value(key):
    value = rpkv.get_from_keystore(key)
    if None is value:
        value = ''
    return value

@api.route('/store', methods=['POST'])
def add_value():
    key = request.args.get('key')
    value = request.args.get('value')

    if None in (key, value):
        return '`query` and `value` are needed in query params', 400

    rpkv.add_to_keystore(key, value)
    return value, 200

if __name__ == '__main__':
    api.run()
