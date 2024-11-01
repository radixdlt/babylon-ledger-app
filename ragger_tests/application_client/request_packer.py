class RequestPacker:
    @classmethod
    def pack_rola_request(self, dapp_def_addr: str, origin: str, nonce: bytes) -> bytes:
        addr_length = len(dapp_def_addr).to_bytes(1, 'little').hex()
        data = nonce + addr_length + dapp_def_addr.encode('utf-8').hex() + origin.encode('utf-8').hex()
        return bytes.fromhex(data)