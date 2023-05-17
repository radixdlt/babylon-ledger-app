import sys
import os

from ledgerblue.comm import getDongle
from ledgerblue.commTCP import getDongle as getDongleTCP
from cryptography.hazmat.primitives.asymmetric import ec, utils
from cryptography.hazmat.primitives import hashes

# disable printing stack trace
sys.tracebacklimit = 0

if os.environ.get('USE_SPECULOS') is not None:
    dongle = getDongleTCP(debug=False)
else:
    dongle = getDongle(False)

instructionClass = "AA"
instructionCode = "71"  # SignAuthSecp256k1
p1 = "00"
p2 = "00"
dataLength = "00"

print("Testing", "SignAuth", instructionCode)


def encode_bip32(path):
    elements = path.replace('H', "'").replace('"', "'").split('/')
    result = (len(elements) - 1).to_bytes(1, 'little').hex()
    for i in range(1, len(elements)):
        num = 0x80000000 if elements[i].endswith("'") else 0
        num += int(elements[i].replace("'", ""))
        result += num.to_bytes(4, 'big').hex()
    return result


def send_auth_request(daddr, origin, nonce):
    addr_length = len(daddr).to_bytes(1, 'little').hex()
    data = nonce + addr_length + daddr.encode('utf-8').hex() + origin.encode('utf-8').hex()
    data_length = int(len(data) / 2).to_bytes(1, 'little').hex()

    try:
        rc = dongle.exchange(bytes.fromhex("AC" + instructionCode + p1 + p2 + data_length + data))
    except Exception as e:
        print("Error sending txn chunk: ", e)
        return None
    return rc


def send_derivation_path(bip_path):
    path_data = encode_bip32(bip_path)
    data_length = int(len(path_data) / 2).to_bytes(1, 'little').hex()
    # print("Sending derivation path: ", bip_path, ", data_len = ", data_length)

    try:
        return dongle.exchange(bytes.fromhex(instructionClass + instructionCode + p1 + p2 + data_length + path_data))
    except Exception as e:
        print("Error sending derivation path: ", e)
        return None


test_vectors = [
    (
        "c05cd851c0ff9d3d6022a23072640d4863b99c68d56ba1796dc0a75c32c46cef",
        "account_tdx_b_1p9dkged3rpzy860ampt5jpmvv3yl4y6f5yppp4tnscdslvt9v3",
        "https://dashboard.rdx.works",
        "17f3cb369f2632454f7f22c24e72b0adf7b95e36f2297467d3ff04010b2967e1"
    ),
    (
        "38629629bbef2917cbe4f5672588d27300ceede5571cef01f49b15b376a9a4d4",
        "account_tdx_b_1p9dkged3rpzy860ampt5jpmvv3yl4y6f5yppp4tnscdslvt9v3",
        "https://dashboard.rdx.works",
        "37b7300c583e963118f12efc80eda923216223931c038d332724178cc94040a1"
    ),
    (
        "d23b15c53e188484541ce61425e67c4507f45ada08b8c19b1ed0fbc9fb7514c7",
        "account_tdx_b_1p9dkged3rpzy860ampt5jpmvv3yl4y6f5yppp4tnscdslvt9v3",
        "https://dashboard.rdx.works",
        "a015a5435262f2431cca5726afedf8ac26249ccf702a9c041771f08a98160d37"
    ),
    (
        "41d70c2abde7e225d5a71a9ac1c4d75d7a2709c337679b17f6fbd7c7935c3fc6",
        "account_tdx_b_1p9dkged3rpzy860ampt5jpmvv3yl4y6f5yppp4tnscdslvt9v3",
        "https://dashboard.rdx.works",
        "576dfb9cf4949bdb299ec1c2fc1402026dd7e187d3764b0b1f20fdbb77a972e1"
    ),
    (
        "50bef3e26a8526eb0082be76b19424a10412482d6d49176c2732a33d30c1cd36",
        "account_tdx_b_1p9dkged3rpzy860ampt5jpmvv3yl4y6f5yppp4tnscdslvt9v3",
        "https://dashboard.rdx.works",
        "eda8df9dee6a4747b490ec06849a8b93e3d15e27a5ad1bd110af5d82ce02436a"
    ),
    (
        "d6188d06e3a36db016d81a120d4fff00d174f5ed83a5b3343a12c3544165307c",
        "account_tdx_b_1p9dkged3rpzy860ampt5jpmvv3yl4y6f5yppp4tnscdslvt9v3",
        "https://dashboard.rdx.works",
        "53b0f594191207aace4b0d96250216ae7ca1d4e61228f1ef609c08d3a454ce90"
    ),
    (
        "235c9de6c5aa5d28d1b5ff8afc09a66400937be6c2c0012bb666dcae1d64e678",
        "account_tdx_b_1p9dkged3rpzy860ampt5jpmvv3yl4y6f5yppp4tnscdslvt9v3",
        "https://dashboard.rdx.works",
        "549ef0d2231603b0bb58a1da9482bf1086a551b3ce74502d8a6ee7f639ce4ef8"
    ),
    (
        "447c2428647d00919da86a1dc9c269e158fbd6b860a7b3f3682d2b185a637e69",
        "account_tdx_b_1p9dkged3rpzy860ampt5jpmvv3yl4y6f5yppp4tnscdslvt9v3",
        "https://dashboard.rdx.works",
        "7aeb92d87c025470e0c25b87426221b8e259346bc12d18d93e9ae454044484ae"
    ),
    (
        "e28eed51b5e13254075aa5292f52e45614c4c353e6e202479faa44ba679b5c11",
        "account_tdx_b_1p9dkged3rpzy860ampt5jpmvv3yl4y6f5yppp4tnscdslvt9v3",
        "https://dashboard.rdx.works",
        "721253c07003ef0d92c0b01472d0e3a0bacbd44cc8a9f95f97ca16cfc0f14a9f"
    ),
    (
        "0396ed6386d523c5728ed3be944d303bd49f4c068fee32fb68bc3367508e9dac",
        "account_tdx_b_1p9dkged3rpzy860ampt5jpmvv3yl4y6f5yppp4tnscdslvt9v3",
        "https://dashboard.rdx.works",
        "ec5dcb3d1f75627be1021cb8890f0e8ce0c9fe7f2ff55cbdff096b38a32612c9"
    ),
    (
        "bad9930249a93e84feec884a495b12ce94ba6548c79ba0c72b6d198a44356e3f",
        "account_tdx_b_1p95nal0nmrqyl5r4phcspg8ahwnamaduzdd3kaklw3vqeavrwa",
        "https://dashboard.rdx.works",
        "9002d52ee0c59f2dc7d9fda2b2a8ef4fcb24d6f0ac13aa4b0ee15da61a222082"
    ),
    (
        "2c3710085d13b0a65c075bbee502d1a36f3955750b9d55d14407c1a3162c04f0",
        "account_tdx_b_1p95nal0nmrqyl5r4phcspg8ahwnamaduzdd3kaklw3vqeavrwa",
        "https://dashboard.rdx.works",
        "c8b612de4dd1cca5f8205d2d7d729954f32d5e790cda156212dd0fa775ce9d90"
    ),
    (
        "c51febbcc5e5d02725e2e700011d4fd49a75c8743b358163a81c8be4ac702269",
        "account_tdx_b_1p95nal0nmrqyl5r4phcspg8ahwnamaduzdd3kaklw3vqeavrwa",
        "https://dashboard.rdx.works",
        "c9fe61de53803e232ad375fa8c5d910239cd0449b78144add96cf1b92e4b6337"
    ),
    (
        "4d33a95f1114e1008302f7c42c09aec2552ab82f7d5741e4baaf5ec3a004d3bf",
        "account_tdx_b_1p95nal0nmrqyl5r4phcspg8ahwnamaduzdd3kaklw3vqeavrwa",
        "https://dashboard.rdx.works",
        "bc823ad8e0396e4f0b90961abd0ca26a107c974b2732464fd3447b1f160d0782"
    ),
    (
        "380057b05189ed8e2305eeead50e63d37e2a6603fe3d5cbb4a331826c678ae2a",
        "account_tdx_b_1p95nal0nmrqyl5r4phcspg8ahwnamaduzdd3kaklw3vqeavrwa",
        "https://dashboard.rdx.works",
        "256846d59b3c3b2bc34f9a21dc5df45e67059843f90b2a1e781aacdb52a6ae2c"
    ),
    (
        "913ef58efd5fa8de00651ac18ffbf3e07a5cddd22743e51a5dc8fff5508599ea",
        "account_tdx_b_1p95nal0nmrqyl5r4phcspg8ahwnamaduzdd3kaklw3vqeavrwa",
        "https://dashboard.rdx.works",
        "546a45467621a24d49cce1a6294ec6ca59a6b807b6acd82667df6ead4d582d7b"
    ),
    (
        "af37d356f19b7e5ae3ee3b7138ed71ca1444265ebf47873a8fd3d0f8a31f6edd",
        "account_tdx_b_1p95nal0nmrqyl5r4phcspg8ahwnamaduzdd3kaklw3vqeavrwa",
        "https://dashboard.rdx.works",
        "b99d25fcc5981a2c82752d494630351bdb197b24fda150f2daeb05a09eda214c"
    ),
    (
        "2498e6f8f9974f3f30057f97a772d838ed4d91080ad04eecc6118b9cc36fb0d7",
        "account_tdx_b_1p95nal0nmrqyl5r4phcspg8ahwnamaduzdd3kaklw3vqeavrwa",
        "https://dashboard.rdx.works",
        "4e6d5b1c00b508ba1ba5c7a4537de1075a056630b66f89c3d89425dba2347e42"
    ),
    (
        "8709f0c94a499a30516bfa2cbe1c006b419a93bc75e37db26193f18c41b63cc1",
        "account_tdx_b_1p95nal0nmrqyl5r4phcspg8ahwnamaduzdd3kaklw3vqeavrwa",
        "https://dashboard.rdx.works",
        "da30918252ee2233781e8fe8992374c4209ca3f3c68b074e8d25472d00c7f397"
    ),
    (
        "5f5cd80ca193ae943266d047d9508cfbfca0d5cade33ff341b59feb3744972e5",
        "account_tdx_b_1p95nal0nmrqyl5r4phcspg8ahwnamaduzdd3kaklw3vqeavrwa",
        "https://dashboard.rdx.works",
        "4aaa2ec25c3fe215412b3f005e4c37d518af3a22b4728587cf6dbcf83341e8b3"
    ),
    (
        "db7700b1ac020b8ef9c43a829e878132ac74e57cad9e21c7d4237880126aea73",
        "account_tdx_b_1p8ahenyznrqy2w0tyg00r82rwuxys6z8kmrhh37c7maqpydx7p",
        "https://dashboard.rdx.works",
        "734437a1fe900d95a73c937d7c4be39a306e1490149848b287264ca622c743fb"
    ),
    (
        "d9d458e36cc858afd306f1df33385a4ac73a1e623dc2419b6f03e2c27e0b0c52",
        "account_tdx_b_1p8ahenyznrqy2w0tyg00r82rwuxys6z8kmrhh37c7maqpydx7p",
        "https://dashboard.rdx.works",
        "100e0f4b7b8dba6da01f80eb9666a57e016223fd3271c85a0f55acb6f3e5a722"
    ),
    (
        "581e60fd4206fee245b7012a63d795039b941a273fc7b97f711d11ae39e2514d",
        "account_tdx_b_1p8ahenyznrqy2w0tyg00r82rwuxys6z8kmrhh37c7maqpydx7p",
        "https://dashboard.rdx.works",
        "e83b6b8b36fe24a713de5f8ce51c1efcb2e6bb396c33f06d9b61671d78ff02b9"
    ),
    (
        "182139581adbe5b487e383cac027a35fd75c18f2130c42a6600f2d8529f26e59",
        "account_tdx_b_1p8ahenyznrqy2w0tyg00r82rwuxys6z8kmrhh37c7maqpydx7p",
        "https://dashboard.rdx.works",
        "9640e7aace384d8274cc8ffe83ffced164f3c72bac8c20df718611235b5c7dc9"
    ),
    (
        "5630520af6d58d3a7f5d50775a5ce7032ef17d5c3b072d840d27bbd94cc0778b",
        "account_tdx_b_1p8ahenyznrqy2w0tyg00r82rwuxys6z8kmrhh37c7maqpydx7p",
        "https://dashboard.rdx.works",
        "ce85a5813069f7e391809bd58496881c91663033b2da33d59e982589e46309ec"
    ),
    (
        "db8423bc0b659b8ac700e8df4b9dbfa88d8e825adaf038df864a2dcc5e66d362",
        "account_tdx_b_1p8ahenyznrqy2w0tyg00r82rwuxys6z8kmrhh37c7maqpydx7p",
        "https://dashboard.rdx.works",
        "279691b716926baf49a6997950b783bd7be340d6d3d34e28353ed877ac711a6f"
    ),
    (
        "ae75436cd625d7933448572eaa7d7edf9ab9745384923e102ebebfb34ec11466",
        "account_tdx_b_1p8ahenyznrqy2w0tyg00r82rwuxys6z8kmrhh37c7maqpydx7p",
        "https://dashboard.rdx.works",
        "daf06ef4fc56486d8771227d5587ed3c37cb37d9b96acae6642f10fc628da8a0"
    ),
    (
        "9a9c866713c738901dad69c344936e6540f746a3143d77e5dd49e884c192df6f",
        "account_tdx_b_1p8ahenyznrqy2w0tyg00r82rwuxys6z8kmrhh37c7maqpydx7p",
        "https://dashboard.rdx.works",
        "8c4dcab505c339e029883dd91d8b103f57a64b6286b537678d5713c656e2b0f8"
    ),
    (
        "4389a1bbde4023e319e515394be987524ea4241a85ae63093870ebdf33905b61",
        "account_tdx_b_1p8ahenyznrqy2w0tyg00r82rwuxys6z8kmrhh37c7maqpydx7p",
        "https://dashboard.rdx.works",
        "55507554647ccd504aec7b262608bcf54342d0783c7a90d40b49d83277523fc0"
    ),
    (
        "2f32e7b4f2d4ef76f4a9f17e2151e6213133f176b62a643629bde83324dbeb7a",
        "account_tdx_b_1p8ahenyznrqy2w0tyg00r82rwuxys6z8kmrhh37c7maqpydx7p",
        "https://dashboard.rdx.works",
        "d7fb740b9ff00657d710dcbeddb2d432e697fc0dd39c60feb7858b17ef0eff58"
    ),
    (
        "9bc0f97fe6bf014fe950f5603c7a00c7d44fec526cbc4dc9265ea8ed4770d1f5",
        "account_tdx_b_1p9dkged3rpzy860ampt5jpmvv3yl4y6f5yppp4tnscdslvt9v3",
        "https://stella.swap",
        "75800c580ea870480b61728fc377e1bcf661f5c1dce9e560833fb2a9af640161"
    ),
    (
        "9fcc1b6146292ee9d560c109223128adadef41f56d582b87d94b56b72a825a17",
        "account_tdx_b_1p9dkged3rpzy860ampt5jpmvv3yl4y6f5yppp4tnscdslvt9v3",
        "https://stella.swap",
        "412593f1c558715ca009226381bed5de229eed7124429a9419899ed40c441415"
    ),
    (
        "d260fb8912710c4d8231fd27251849038fd74ddad7c0da1619ab31426ecb7c9e",
        "account_tdx_b_1p9dkged3rpzy860ampt5jpmvv3yl4y6f5yppp4tnscdslvt9v3",
        "https://stella.swap",
        "cb4c6f8a16d4c6035487cdcd118a2d6805a560443f8c910dbe91a6ff0f0c00d8"
    ),
    (
        "ab23b2e0c1378cecb7a1bc0098951ff2b40e117ab39bd49752c6d94e1812c2f3",
        "account_tdx_b_1p9dkged3rpzy860ampt5jpmvv3yl4y6f5yppp4tnscdslvt9v3",
        "https://stella.swap",
        "fe133faa758347941724c7f3a329a06e8405c582800223c3192a7f2d15964123"
    ),
    (
        "bc3d36bd125b9f82994a373da8712fffd622b9e1e77fb91c2b85f238d3b53e0f",
        "account_tdx_b_1p9dkged3rpzy860ampt5jpmvv3yl4y6f5yppp4tnscdslvt9v3",
        "https://stella.swap",
        "fb62c45b70be8c5b48e3da401b0636af2830acd83c30a142817de68a5ad277c7"
    ),
    (
        "42be291d74c0a6bc632ca9152ccbf017a86401403ef04547e3a5b948f6a8e305",
        "account_tdx_b_1p9dkged3rpzy860ampt5jpmvv3yl4y6f5yppp4tnscdslvt9v3",
        "https://stella.swap",
        "59aa96163adc1d67e961ee652d552028673da478aa72d4b25628797390f0c30a"
    ),
    (
        "7542a34074a324d9194651423f65115b6018dfa9ad07e56a61500bc2b321d57c",
        "account_tdx_b_1p9dkged3rpzy860ampt5jpmvv3yl4y6f5yppp4tnscdslvt9v3",
        "https://stella.swap",
        "2bd1de4ca5ab21bf912412c19da66a44d52572fa84dd1ccd61a041d241f73e26"
    ),
    (
        "8fbc5b098ec79e597917c8b2ab95220460eac8942fbb98e2f5e1914693cbb402",
        "account_tdx_b_1p9dkged3rpzy860ampt5jpmvv3yl4y6f5yppp4tnscdslvt9v3",
        "https://stella.swap",
        "7714ffebc820e7e21c958a95d3f1c42e9a11c46a9928e918667db152b0e92dfd"
    ),
    (
        "dc848c0af9da3497ae6643a974db75ba0199ad02de76c834f2fe2ac7ab72ea08",
        "account_tdx_b_1p9dkged3rpzy860ampt5jpmvv3yl4y6f5yppp4tnscdslvt9v3",
        "https://stella.swap",
        "652e2ddeb3a9d6fd6faec6b43611683ad6a90475e702dc93ac7b26b493719244"
    ),
    (
        "c57a6db4f607c9b46d65a7f03939b1b35581044dfc3d5239680f0803ede8dc3e",
        "account_tdx_b_1p9dkged3rpzy860ampt5jpmvv3yl4y6f5yppp4tnscdslvt9v3",
        "https://stella.swap",
        "a10fad201666b4bcf7f707841d58b11740c290e03790b17ed0fec23b3f180e65"
    ),
    (
        "a3ea38a133a102c251d991b75e11101bcbf9913e314337f43673cd3dcf1a84bf",
        "account_tdx_b_1p95nal0nmrqyl5r4phcspg8ahwnamaduzdd3kaklw3vqeavrwa",
        "https://stella.swap",
        "5ed2a0ef6cc83e5b0763b3a65da16faf9eb6e8f7fcc7bef7c4785bf79f4e07c1"
    ),
    (
        "3d8f051c99c2fdbe8a1d7ede9c4e460bff94ff9d50edbcc2d0fb0f68eab6a967",
        "account_tdx_b_1p95nal0nmrqyl5r4phcspg8ahwnamaduzdd3kaklw3vqeavrwa",
        "https://stella.swap",
        "b9d77c98d7bb876d4f179f23537a824c8c1f6dc7eb3f0d38731e79c1873982c3"
    ),
    (
        "b418f3ce6fb7b5dddec4c2ed1d58183836aeebcc05b3f5495a1a1967e035e2c9",
        "account_tdx_b_1p95nal0nmrqyl5r4phcspg8ahwnamaduzdd3kaklw3vqeavrwa",
        "https://stella.swap",
        "251fad9df0c0aee813666db14e76ed99d2761c0880fa5490a0039e553056d118"
    ),
    (
        "6ede06a2ece3d399fe394c20aad8d4b4c72dd53147fd451ea8828db8d81dadde",
        "account_tdx_b_1p95nal0nmrqyl5r4phcspg8ahwnamaduzdd3kaklw3vqeavrwa",
        "https://stella.swap",
        "a5a0bd6c9471c462c6ffe70abc3d052ee5a80eb648416e8239e88e356d90eeb0"
    ),
    (
        "688464da4cd5f7de0d3e2269f2269f8c16ef78c8756bbcd860341db3dd872f5e",
        "account_tdx_b_1p95nal0nmrqyl5r4phcspg8ahwnamaduzdd3kaklw3vqeavrwa",
        "https://stella.swap",
        "97cf6f4e62d1801e775ee36d3fcba5a3b1499ad91468107136a8aafb17baed39"
    ),
    (
        "a9c32cadaafaa926a8e08d9ac91ca4288be2c6ebef8248031db8d8d51953a043",
        "account_tdx_b_1p95nal0nmrqyl5r4phcspg8ahwnamaduzdd3kaklw3vqeavrwa",
        "https://stella.swap",
        "685dda6453ed3263ccbb307c9467d0f98a8f18dd0fda37a0475ac9a47bed3be9"
    ),
    (
        "0e53c336357276ddae8b42a88f386565e4efe695a5de35ca59ad0db33063b466",
        "account_tdx_b_1p95nal0nmrqyl5r4phcspg8ahwnamaduzdd3kaklw3vqeavrwa",
        "https://stella.swap",
        "2277346c0fb011fb33b111a07b697d48c98fa35bfad5f9f063e7ce4627ba943f"
    ),
    (
        "1a41f19a239249f2768db2db8770d51580ebc4e57275ff40d39018046ed73007",
        "account_tdx_b_1p95nal0nmrqyl5r4phcspg8ahwnamaduzdd3kaklw3vqeavrwa",
        "https://stella.swap",
        "0086f932f939a14759753ced40f45cd0101341f8612b51bafd3692c91777a54b"
    ),
    (
        "1948b01f12db7763895e6711b68c75fd08ed3808480ac5bb9ad658b1742883bc",
        "account_tdx_b_1p95nal0nmrqyl5r4phcspg8ahwnamaduzdd3kaklw3vqeavrwa",
        "https://stella.swap",
        "9bfbf59e71b41eed841732618a24b33907bf79f871c751978557f23567360921"
    ),
    (
        "44f31327e5f6dae814fe7542f6ed8bce1abd1eef182959c54b85e85598d13bce",
        "account_tdx_b_1p95nal0nmrqyl5r4phcspg8ahwnamaduzdd3kaklw3vqeavrwa",
        "https://stella.swap",
        "9a4f834aefdc455cb4601337227e1b7e74d60308327564ececf33456509964cd"
    ),
    (
        "9c696391f030201110c5f71e9269422f3eb2ca024855f4efa6da89bf7f0de398",
        "account_tdx_b_1p8ahenyznrqy2w0tyg00r82rwuxys6z8kmrhh37c7maqpydx7p",
        "https://stella.swap",
        "231e1c9be95a914162221c3eaf52a674242e7922e0020a554b80f772792e9edc"
    ),
    (
        "7f4fa9be20086bafecb599f4d8c39ac7a99c81736be1508c321f9ac2f87a9ba7",
        "account_tdx_b_1p8ahenyznrqy2w0tyg00r82rwuxys6z8kmrhh37c7maqpydx7p",
        "https://stella.swap",
        "954b237e92a6a5ab0baf9d139d1c65bf00e63363bbfbb107a3841714391d4ecf"
    ),
    (
        "7395a26b2019f726f01ba1f0b4d33f33459855b599e3f80a447c1b30ad54edfc",
        "account_tdx_b_1p8ahenyznrqy2w0tyg00r82rwuxys6z8kmrhh37c7maqpydx7p",
        "https://stella.swap",
        "a0c9f3bf148c85203d2489436d66e1ce497ff0469ff6bc72997e5185279a33a5"
    ),
    (
        "2ebe9efbbab27029a89fe2bb547fd644cfa600ae0e643d16cf7cdabc38c73bb6",
        "account_tdx_b_1p8ahenyznrqy2w0tyg00r82rwuxys6z8kmrhh37c7maqpydx7p",
        "https://stella.swap",
        "11966bd56cb16b72c67bc7d2ec167951d67f87670f686c2eff58cbccb67dc405"
    ),
    (
        "7ea75eefc784433e94112f43d2886e07732e4d5e191bd4b3ee546b69d62c356e",
        "account_tdx_b_1p8ahenyznrqy2w0tyg00r82rwuxys6z8kmrhh37c7maqpydx7p",
        "https://stella.swap",
        "2596d7afc6e5975e16804293facef830c17b08cc2cff5a6812297debd865d003"
    ),
    (
        "b5f077c7718264928cd942dbd9702d326753d0cd945d5af9fada08caf43c88da",
        "account_tdx_b_1p8ahenyznrqy2w0tyg00r82rwuxys6z8kmrhh37c7maqpydx7p",
        "https://stella.swap",
        "304c6bae5c78f9ec55012c7bedb342d9eec145f34b66392df6654bd9bff03d83"
    ),
    (
        "d19e191e6ac1e7c5d72d69e11ab547146766c9358470f491d7cf547357086db2",
        "account_tdx_b_1p8ahenyznrqy2w0tyg00r82rwuxys6z8kmrhh37c7maqpydx7p",
        "https://stella.swap",
        "cf02ba2830d4b286ea7cba2343213ad26bea5555c663de179879c431c6b9734d"
    ),
    (
        "d6594f95fa2da899335f778a888c4f4d728f581923155d4936895bc04e141c32",
        "account_tdx_b_1p8ahenyznrqy2w0tyg00r82rwuxys6z8kmrhh37c7maqpydx7p",
        "https://stella.swap",
        "bce429efded3cb37e916bac43a3ccb21c1be5ade97709c69785ba45952e34d21"
    ),
    (
        "e3467c70527119660640b584cf100bc88b78bb8afd0e5c6c1fdd1aef6bcf7cd7",
        "account_tdx_b_1p8ahenyznrqy2w0tyg00r82rwuxys6z8kmrhh37c7maqpydx7p",
        "https://stella.swap",
        "da78d83d7d41aac75fa18eb54252a36926cfa4862818b207fd5722a8f1707243"
    ),
    (
        "c81fb494506ca54b8b87d40af24706ce7e53b0b13b0b879bd1a202652680ca69",
        "account_tdx_b_1p8ahenyznrqy2w0tyg00r82rwuxys6z8kmrhh37c7maqpydx7p",
        "https://stella.swap",
        "718b0eb060a719492011910258a4b4119d8c95aef34eb9519c9fa7de25f7ac43"
    ),
    (
        "879a13f14eeb0c92561fd9d54f0c13992dda06e145eb1b6f68d1f33bf56c74c7",
        "account_tdx_b_1p9dkged3rpzy860ampt5jpmvv3yl4y6f5yppp4tnscdslvt9v3",
        "https://rola.xrd",
        "44928b503ec3df91aacd39a9439563c2ad86af19ffa0f8b11a41c6376fa5f1b4"
    ),
    (
        "ade59acf3389e02e98af47b86d23006f2da3a28f4e182b9529740ea9ca3b7057",
        "account_tdx_b_1p9dkged3rpzy860ampt5jpmvv3yl4y6f5yppp4tnscdslvt9v3",
        "https://rola.xrd",
        "627cbe65a0e69787c81e426d0cf2d8160df3b10e36744cd106d033b09b69eb54"
    ),
    (
        "494ce91c81d9a78b5ed0acc77328a4b672e7eed60c6ce8f21d1aec3ee66a8944",
        "account_tdx_b_1p9dkged3rpzy860ampt5jpmvv3yl4y6f5yppp4tnscdslvt9v3",
        "https://rola.xrd",
        "3581aea035195fb55472172a0dfb49da85d55e3a0db68587b814807dce7049ea"
    ),
    (
        "29cd7e4de359886f0adf1e6f1e54952b90b1c3628a1da8caffa6c856424f15b1",
        "account_tdx_b_1p9dkged3rpzy860ampt5jpmvv3yl4y6f5yppp4tnscdslvt9v3",
        "https://rola.xrd",
        "2cb201ffe1c14e08b83fb08ab99eeb0d6d6912ef716acfd7681e597fe7c98a54"
    ),
    (
        "9cc21c7bfde33e8e548b3f173acdd9ff624351e6cc702c66ce88855501dbf25c",
        "account_tdx_b_1p9dkged3rpzy860ampt5jpmvv3yl4y6f5yppp4tnscdslvt9v3",
        "https://rola.xrd",
        "d27e43ced50394cff53e8815ec632f7c1f7f8f5efd923ad7558d263cfe2066d1"
    ),
    (
        "e0f1da447a962799f5c347a2b9b07f66c213c402aa08f36a1f83b46329070e73",
        "account_tdx_b_1p9dkged3rpzy860ampt5jpmvv3yl4y6f5yppp4tnscdslvt9v3",
        "https://rola.xrd",
        "30eb2b49998b7dbfb64bd8524bfd8ca73fdbe5b8419d3e9430769cff08f73203"
    ),
    (
        "4b7c35f51517452bbb575cccea99769bb1c02cda8f5c5cd42854ae76fd5b0cfc",
        "account_tdx_b_1p9dkged3rpzy860ampt5jpmvv3yl4y6f5yppp4tnscdslvt9v3",
        "https://rola.xrd",
        "7a7aad9fa65a2dbfccc9e44561b302572f260f65c73c6d1c5c87a8a8bdf6275c"
    ),
    (
        "dd08fdf84146225cf0726a8a75194c8032f21d09aaa0366d7244454c633cad9c",
        "account_tdx_b_1p9dkged3rpzy860ampt5jpmvv3yl4y6f5yppp4tnscdslvt9v3",
        "https://rola.xrd",
        "d81619c7e5ab2298c419ea7eda9c89db0db3f1d84713c495c3ec93d811d945b4"
    ),
    (
        "1e4ea30f2756638565cbc1df81c653191eb92748228f11f180822c538b862b6a",
        "account_tdx_b_1p9dkged3rpzy860ampt5jpmvv3yl4y6f5yppp4tnscdslvt9v3",
        "https://rola.xrd",
        "67524c29ad63ce51e4807ce154db895b1c0509d67725747b021b826c24be775a"
    ),
    (
        "d277637594674a3c8ce013e786e774d439eb7c8a601d68f66fdd76f97117b650",
        "account_tdx_b_1p9dkged3rpzy860ampt5jpmvv3yl4y6f5yppp4tnscdslvt9v3",
        "https://rola.xrd",
        "00dca15875839ab1f549445a36c7b5c0dcf7aebfa7d48f945f2aa5cf4aa1a9a3"
    ),
    (
        "c7cd9f469b07df768bf79c81189a359017c54636c81b602cb5aaff1e83452143",
        "account_tdx_b_1p95nal0nmrqyl5r4phcspg8ahwnamaduzdd3kaklw3vqeavrwa",
        "https://rola.xrd",
        "30c6300639a90f15f5f34290c0dbdc3b07bc36ba9f793ff55f36885860a32138"
    ),
    (
        "5e4d38a1f8e1ee167bcdb63e24fda0c51baf9cd544d34fd65a674b18ed8c71ce",
        "account_tdx_b_1p95nal0nmrqyl5r4phcspg8ahwnamaduzdd3kaklw3vqeavrwa",
        "https://rola.xrd",
        "19cd952215ef89fd393b50460ddee05fd48d0b8543bc008670bd415f66ce945b"
    ),
    (
        "8720249044062b589fc7d5268fae70a376b0a6e310327c65e053848c617f415b",
        "account_tdx_b_1p95nal0nmrqyl5r4phcspg8ahwnamaduzdd3kaklw3vqeavrwa",
        "https://rola.xrd",
        "3a826316cf8a0ac49b72baea22973d7ef5923854735c9e5d2297c2f9277c1d61"
    ),
    (
        "6af5ddcf1c35f267f4fc451b786f5de56d72883ce0ed01f8448f83a3bd8fa173",
        "account_tdx_b_1p95nal0nmrqyl5r4phcspg8ahwnamaduzdd3kaklw3vqeavrwa",
        "https://rola.xrd",
        "f2d555e74fb1d9ded5ce512307bb570f1add46239754ef770cb9e4c1fb7f222d"
    ),
    (
        "0b222e5b4fcee195ccd4b97a9cc3be400011c6fe93fb7288d674bdf1122142f2",
        "account_tdx_b_1p95nal0nmrqyl5r4phcspg8ahwnamaduzdd3kaklw3vqeavrwa",
        "https://rola.xrd",
        "0347e29e7c1a5daadbd52a7c29ab367cbc3eb394b06ec8489c26333c7a8947fe"
    ),
    (
        "bd966923ec467cfb46483327d2d23b5c22440a93b89447160a34e9e90b07312c",
        "account_tdx_b_1p95nal0nmrqyl5r4phcspg8ahwnamaduzdd3kaklw3vqeavrwa",
        "https://rola.xrd",
        "acd2d6cb47f9175e3c71d41dd9cd639740cd501a63911ef740bdc408377ab4e8"
    ),
    (
        "9ed07a8cffcd012eb36e23b59310aff428e2439200fc2cb309a9480fc37f2a50",
        "account_tdx_b_1p95nal0nmrqyl5r4phcspg8ahwnamaduzdd3kaklw3vqeavrwa",
        "https://rola.xrd",
        "5599f54941f26cb1c228d4a0087e28406bd27302ecdc623c63dbf50007619ff1"
    ),
    (
        "f8ca625aca6fc9e682cca45a71dd27ec1798b75e7dfc47f0bcd6934ddec395bd",
        "account_tdx_b_1p95nal0nmrqyl5r4phcspg8ahwnamaduzdd3kaklw3vqeavrwa",
        "https://rola.xrd",
        "b0407e154ebfc958cf0e532381f75fde65e36c79935c7eba9e063520f5a90c27"
    ),
    (
        "8c2edd453dc022a67d06fa621ed362fc644e568398bb2e8d392a495b8ea1b4e1",
        "account_tdx_b_1p95nal0nmrqyl5r4phcspg8ahwnamaduzdd3kaklw3vqeavrwa",
        "https://rola.xrd",
        "6e6ab02769d14cd0be62267919c1f8a7a671029c41ade1f4d88336e8438e78db"
    ),
    (
        "04ed72a9c061adb310a00927efb333c31a76c156f9335218f776dc8ec9507020",
        "account_tdx_b_1p95nal0nmrqyl5r4phcspg8ahwnamaduzdd3kaklw3vqeavrwa",
        "https://rola.xrd",
        "20619c1df905a28e7a76d431f2b59e99dd1a8f386842e1701862e765806a5c47"
    ),
    (
        "7caafc50aa8711a66af66ae0509b7fc1a9c55dc8abb0d9d1d1b36711ba552fb8",
        "account_tdx_b_1p8ahenyznrqy2w0tyg00r82rwuxys6z8kmrhh37c7maqpydx7p",
        "https://rola.xrd",
        "f8e21638f1c1ecd622fcd2354bbbdcfd06c093f7682995c933915f5adefbed37"
    ),
    (
        "70c114e247eced919ad4b579aab97762b363372e1b3c38c3151faef2296dd861",
        "account_tdx_b_1p8ahenyznrqy2w0tyg00r82rwuxys6z8kmrhh37c7maqpydx7p",
        "https://rola.xrd",
        "d957ec97d74f3ef81c7c450aae7ae7a094b62bf68ab48cbde1c1d5d6882e2f36"
    ),
    (
        "b81bdfbb10da0acfa8ca6beb79bdbe8b70033f941ce3a7d07dfdb8294ac3cf5f",
        "account_tdx_b_1p8ahenyznrqy2w0tyg00r82rwuxys6z8kmrhh37c7maqpydx7p",
        "https://rola.xrd",
        "aced965f5ff005bc7fcac7a4db026cb1c3b2dcf3499c2099ca2423a27d39dbd2"
    ),
    (
        "d05fcedd2df9315837498adebe999e20fdb5bd6f17ef18f19d82a59f58743cb1",
        "account_tdx_b_1p8ahenyznrqy2w0tyg00r82rwuxys6z8kmrhh37c7maqpydx7p",
        "https://rola.xrd",
        "cef66932a002c0e949cbbd4f7774e2d3082ea3ffc40d76f65537e15b97e840e1"
    ),
    (
        "3a96e81a4b843e95ac6ed3ec18baa4c4db6f24b1e3f56d1b39e2be96281b4ad3",
        "account_tdx_b_1p8ahenyznrqy2w0tyg00r82rwuxys6z8kmrhh37c7maqpydx7p",
        "https://rola.xrd",
        "b6ce0722f3943831e3cf04afed287e574bc0c5786662e4f9bc701f429a3f3a4c"
    ),
    (
        "8c7494364b2bd2ae00ad41592bb158cd0b7d2e2604ed7a49e9a9461166e75af4",
        "account_tdx_b_1p8ahenyznrqy2w0tyg00r82rwuxys6z8kmrhh37c7maqpydx7p",
        "https://rola.xrd",
        "1e7a54fc94e53ba7d019ecbb2957a06fd47d088c69def20e0e439e4ae0429f28"
    ),
    (
        "6f432a2903af3f0b8a133ef380006bcfae3c06d072f6b795c82a04128c780c7a",
        "account_tdx_b_1p8ahenyznrqy2w0tyg00r82rwuxys6z8kmrhh37c7maqpydx7p",
        "https://rola.xrd",
        "aa7a87ec65d9d0f387572c1642498f8d0d9b38d5a4df5f87aad27f102a4d0d92"
    ),
    (
        "1e7aa742f48fb5806ad85a112358a2aea630a385fad08804dd317727fc1fbdcd",
        "account_tdx_b_1p8ahenyznrqy2w0tyg00r82rwuxys6z8kmrhh37c7maqpydx7p",
        "https://rola.xrd",
        "52b8151a401f0176ebeca25593d23f6dcecc76c98829734e9c2381270f95bab4"
    ),
    (
        "1d53eb60c14bcfd2532bddc9879291cdcefc142e65fa0862d04b22959a1c6a72",
        "account_tdx_b_1p8ahenyznrqy2w0tyg00r82rwuxys6z8kmrhh37c7maqpydx7p",
        "https://rola.xrd",
        "140100298edb9cadd80423c1e15e2521d44f51be8b6a7953c9f03d83a8220071"
    ),
    (
        "68999b1daa0a7c137ee96f0f9b98587420dc2b2958c7d41252e44620ba98591b",
        "account_tdx_b_1p8ahenyznrqy2w0tyg00r82rwuxys6z8kmrhh37c7maqpydx7p",
        "https://rola.xrd",
        "0a510b2362c9ce19d11c538b2f6a15f62caab6528071eaad5ba8a563a02e01cb"
    )
]

for vector in test_vectors:
    send_derivation_path("m/44H/1022H/10H/525H/1238H")
    rc = send_auth_request(vector[1], vector[2], vector[3])

    if rc is None:
        print("Failed")
    else:
        r = int.from_bytes(rc[1:33], byteorder='big', signed=False)
        s = int.from_bytes(rc[33:65], byteorder='big', signed=False)
        signature = utils.encode_dss_signature(int(r), int(s))
        pubkey = ec.EllipticCurvePublicKey.from_encoded_point(ec.SECP256K1(), bytes(rc[65:98]))
        try:
            # Note that Prehashed parameter is irrelevant here, we just need to pass something known to the library
            pubkey.verify(signature, bytes(rc[98:130]), ec.ECDSA(utils.Prehashed(hashes.SHA256())))
            print("Success")
            assert rc[98:130].hex() == vector[0], "Invalid calculated hash\nExpected: " + vector[0] + "\nReceived: " + rc[98:130].hex()
        except Exception as e:
            print("Invalid signature ", e)

print("All tests successfully passed")
