/*
 * Copyright 2013 Jack Lloyd
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use hash::*;

#[allow(non_camel_case_types)]
struct HMAC_CTX {
    mut md: EVP_MD,
    mut md_ctx: EVP_MD_CTX,
    mut i_ctx: EVP_MD_CTX,
    mut o_ctx: EVP_MD_CTX,
    mut key_length: libc::c_uint,
    mut key: [libc::c_uchar * 128]
}

#[link_name = "crypto"]
#[abi = "cdecl"]
extern mod libcrypto {

    fn HMAC_CTX_init(ctx: *mut HMAC_CTX, key: *u8, keylen: libc::c_int, md: EVP_MD);

    fn HMAC_Update(ctx: *mut HMAC_CTX, input: *u8, len: libc::c_uint);

    fn HMAC_Final(ctx: *mut HMAC_CTX, output: *mut u8, len: *mut libc::c_uint);
}

pub struct HMAC {
    priv mut ctx: HMAC_CTX,
    priv len: uint,
}

pub fn HMAC(ht: HashType, key: ~[u8]) -> HMAC {
    unsafe {

        let (evp, mdlen) = evpmd(ht);

        let mut ctx : HMAC_CTX = HMAC_CTX {
            mut md: ptr::null(),
            mut md_ctx: ptr::null(),
            mut i_ctx: ptr::null(),
            mut o_ctx: ptr::null(),
            mut key_length: 0,
            mut key: [0u8, .. 128]
        };

        libcrypto::HMAC_CTX_init(&mut ctx,
                                 vec::raw::to_ptr(key),
                                 key.len() as libc::c_int,
                                 evp);

        HMAC { ctx: ctx, len: mdlen }
    }
}

pub impl HMAC {
    fn update(data: &[u8]) unsafe {
        do vec::as_imm_buf(data) |pdata, len| {
            libcrypto::HMAC_Update(&mut self.ctx, pdata, len as libc::c_uint)
        }
    }

    fn final() -> ~[u8] unsafe {
        let mut res = vec::from_elem(self.len, 0u8);
        let mut outlen: libc::c_uint = 0;
        do vec::as_mut_buf(res) |pres, _len| {
            libcrypto::HMAC_Final(&mut self.ctx, pres, &mut outlen);
            assert self.len == outlen as uint
        }
        res
    }
}

fn main() {
    let h = HMAC(SHA512, ~[00u8]);

    h.update(~[00u8]);

    io::println(fmt!("%?", h.final()))
}