#[cfg(all(not(feature = "openssl"), feature = "wasmcomponent"))]
mod boringsslwasmcomp {
    #[allow(dead_code, clippy::all)]
    pub mod cm {
        pub mod wasmquiche {
            #[allow(dead_code, clippy::all)]
            pub mod boringssl {
                #[used]
                #[doc(hidden)]
                static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
                use super::super::super::_rt;
                pub type UintptrT = u32;
                /// --
                pub enum QuicError {
                    Done,
                    Buffertooshort,
                    Unknownversion,
                    Invalidframe,
                    Invalidpacket,
                    Invalidstate,
                    Invalidstreamstate(u64),
                    Invalidtransportparam,
                    Cryptofail,
                    Tlsfail,
                    Flowcontrol,
                    Streamlimit,
                    Streamstopped(u64),
                    Streamreset(u64),
                    Finalsize,
                    Congestioncontrol,
                    Idlimit,
                    Outofidentifiers,
                    Keyupdate,
                    Cryptobufferexceeded,
                }
                #[automatically_derived]
                impl ::core::clone::Clone for QuicError {
                    #[inline]
                    fn clone(&self) -> QuicError {
                        let _: ::core::clone::AssertParamIsClone<u64>;
                        *self
                    }
                }
                #[automatically_derived]
                impl ::core::marker::Copy for QuicError {}
                impl ::core::fmt::Debug for QuicError {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        match self {
                            QuicError::Done => f.debug_tuple("QuicError::Done").finish(),
                            QuicError::Buffertooshort => {
                                f.debug_tuple("QuicError::Buffertooshort").finish()
                            }
                            QuicError::Unknownversion => {
                                f.debug_tuple("QuicError::Unknownversion").finish()
                            }
                            QuicError::Invalidframe => {
                                f.debug_tuple("QuicError::Invalidframe").finish()
                            }
                            QuicError::Invalidpacket => {
                                f.debug_tuple("QuicError::Invalidpacket").finish()
                            }
                            QuicError::Invalidstate => {
                                f.debug_tuple("QuicError::Invalidstate").finish()
                            }
                            QuicError::Invalidstreamstate(e) => {
                                f.debug_tuple("QuicError::Invalidstreamstate")
                                    .field(e)
                                    .finish()
                            }
                            QuicError::Invalidtransportparam => {
                                f.debug_tuple("QuicError::Invalidtransportparam").finish()
                            }
                            QuicError::Cryptofail => {
                                f.debug_tuple("QuicError::Cryptofail").finish()
                            }
                            QuicError::Tlsfail => {
                                f.debug_tuple("QuicError::Tlsfail").finish()
                            }
                            QuicError::Flowcontrol => {
                                f.debug_tuple("QuicError::Flowcontrol").finish()
                            }
                            QuicError::Streamlimit => {
                                f.debug_tuple("QuicError::Streamlimit").finish()
                            }
                            QuicError::Streamstopped(e) => {
                                f.debug_tuple("QuicError::Streamstopped").field(e).finish()
                            }
                            QuicError::Streamreset(e) => {
                                f.debug_tuple("QuicError::Streamreset").field(e).finish()
                            }
                            QuicError::Finalsize => {
                                f.debug_tuple("QuicError::Finalsize").finish()
                            }
                            QuicError::Congestioncontrol => {
                                f.debug_tuple("QuicError::Congestioncontrol").finish()
                            }
                            QuicError::Idlimit => {
                                f.debug_tuple("QuicError::Idlimit").finish()
                            }
                            QuicError::Outofidentifiers => {
                                f.debug_tuple("QuicError::Outofidentifiers").finish()
                            }
                            QuicError::Keyupdate => {
                                f.debug_tuple("QuicError::Keyupdate").finish()
                            }
                            QuicError::Cryptobufferexceeded => {
                                f.debug_tuple("QuicError::Cryptobufferexceeded").finish()
                            }
                        }
                    }
                }
                impl ::core::fmt::Display for QuicError {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        f.write_fmt(format_args!("{0:?}", self))
                    }
                }
                impl std::error::Error for QuicError {}
                #[repr(u8)]
                pub enum Level {
                    Initial,
                    ZeroRtt,
                    Handshake,
                    OneRtt,
                }
                #[automatically_derived]
                impl ::core::clone::Clone for Level {
                    #[inline]
                    fn clone(&self) -> Level {
                        *self
                    }
                }
                #[automatically_derived]
                impl ::core::marker::Copy for Level {}
                #[automatically_derived]
                impl ::core::cmp::Eq for Level {
                    #[inline]
                    #[doc(hidden)]
                    #[coverage(off)]
                    fn assert_receiver_is_total_eq(&self) -> () {}
                }
                #[automatically_derived]
                impl ::core::cmp::Ord for Level {
                    #[inline]
                    fn cmp(&self, other: &Level) -> ::core::cmp::Ordering {
                        let __self_discr = ::core::intrinsics::discriminant_value(self);
                        let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                        ::core::cmp::Ord::cmp(&__self_discr, &__arg1_discr)
                    }
                }
                #[automatically_derived]
                impl ::core::marker::StructuralPartialEq for Level {}
                #[automatically_derived]
                impl ::core::cmp::PartialEq for Level {
                    #[inline]
                    fn eq(&self, other: &Level) -> bool {
                        let __self_discr = ::core::intrinsics::discriminant_value(self);
                        let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                        __self_discr == __arg1_discr
                    }
                }
                #[automatically_derived]
                impl ::core::cmp::PartialOrd for Level {
                    #[inline]
                    fn partial_cmp(
                        &self,
                        other: &Level,
                    ) -> ::core::option::Option<::core::cmp::Ordering> {
                        let __self_discr = ::core::intrinsics::discriminant_value(self);
                        let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                        ::core::cmp::PartialOrd::partial_cmp(
                            &__self_discr,
                            &__arg1_discr,
                        )
                    }
                }
                impl ::core::fmt::Debug for Level {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        match self {
                            Level::Initial => f.debug_tuple("Level::Initial").finish(),
                            Level::ZeroRtt => f.debug_tuple("Level::ZeroRtt").finish(),
                            Level::Handshake => {
                                f.debug_tuple("Level::Handshake").finish()
                            }
                            Level::OneRtt => f.debug_tuple("Level::OneRtt").finish(),
                        }
                    }
                }
                impl Level {
                    #[doc(hidden)]
                    pub unsafe fn _lift(val: u8) -> Level {
                        if !true {
                            return ::core::mem::transmute(val);
                        }
                        match val {
                            0 => Level::Initial,
                            1 => Level::ZeroRtt,
                            2 => Level::Handshake,
                            3 => Level::OneRtt,
                            _ => {
                                ::core::panicking::panic_fmt(
                                    format_args!("invalid enum discriminant"),
                                );
                            }
                        }
                    }
                }
                #[repr(u8)]
                pub enum Algorithm {
                    Aes128Gcm,
                    Aes256Gcm,
                    Chacha20Ploy1305,
                }
                #[automatically_derived]
                impl ::core::clone::Clone for Algorithm {
                    #[inline]
                    fn clone(&self) -> Algorithm {
                        *self
                    }
                }
                #[automatically_derived]
                impl ::core::marker::Copy for Algorithm {}
                #[automatically_derived]
                impl ::core::cmp::Eq for Algorithm {
                    #[inline]
                    #[doc(hidden)]
                    #[coverage(off)]
                    fn assert_receiver_is_total_eq(&self) -> () {}
                }
                #[automatically_derived]
                impl ::core::cmp::Ord for Algorithm {
                    #[inline]
                    fn cmp(&self, other: &Algorithm) -> ::core::cmp::Ordering {
                        let __self_discr = ::core::intrinsics::discriminant_value(self);
                        let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                        ::core::cmp::Ord::cmp(&__self_discr, &__arg1_discr)
                    }
                }
                #[automatically_derived]
                impl ::core::marker::StructuralPartialEq for Algorithm {}
                #[automatically_derived]
                impl ::core::cmp::PartialEq for Algorithm {
                    #[inline]
                    fn eq(&self, other: &Algorithm) -> bool {
                        let __self_discr = ::core::intrinsics::discriminant_value(self);
                        let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                        __self_discr == __arg1_discr
                    }
                }
                #[automatically_derived]
                impl ::core::cmp::PartialOrd for Algorithm {
                    #[inline]
                    fn partial_cmp(
                        &self,
                        other: &Algorithm,
                    ) -> ::core::option::Option<::core::cmp::Ordering> {
                        let __self_discr = ::core::intrinsics::discriminant_value(self);
                        let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                        ::core::cmp::PartialOrd::partial_cmp(
                            &__self_discr,
                            &__arg1_discr,
                        )
                    }
                }
                impl ::core::fmt::Debug for Algorithm {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        match self {
                            Algorithm::Aes128Gcm => {
                                f.debug_tuple("Algorithm::Aes128Gcm").finish()
                            }
                            Algorithm::Aes256Gcm => {
                                f.debug_tuple("Algorithm::Aes256Gcm").finish()
                            }
                            Algorithm::Chacha20Ploy1305 => {
                                f.debug_tuple("Algorithm::Chacha20Ploy1305").finish()
                            }
                        }
                    }
                }
                impl Algorithm {
                    #[doc(hidden)]
                    pub unsafe fn _lift(val: u8) -> Algorithm {
                        if !true {
                            return ::core::mem::transmute(val);
                        }
                        match val {
                            0 => Algorithm::Aes128Gcm,
                            1 => Algorithm::Aes256Gcm,
                            2 => Algorithm::Chacha20Ploy1305,
                            _ => {
                                ::core::panicking::panic_fmt(
                                    format_args!("invalid enum discriminant"),
                                );
                            }
                        }
                    }
                }
                pub struct EvpAeadCtx {
                    pub aead: UintptrT,
                    pub opaque: _rt::Vec<u8>,
                    pub alignment: u64,
                    pub tag_len: u8,
                }
                #[automatically_derived]
                impl ::core::clone::Clone for EvpAeadCtx {
                    #[inline]
                    fn clone(&self) -> EvpAeadCtx {
                        EvpAeadCtx {
                            aead: ::core::clone::Clone::clone(&self.aead),
                            opaque: ::core::clone::Clone::clone(&self.opaque),
                            alignment: ::core::clone::Clone::clone(&self.alignment),
                            tag_len: ::core::clone::Clone::clone(&self.tag_len),
                        }
                    }
                }
                impl ::core::fmt::Debug for EvpAeadCtx {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        f.debug_struct("EvpAeadCtx")
                            .field("aead", &self.aead)
                            .field("opaque", &self.opaque)
                            .field("alignment", &self.alignment)
                            .field("tag-len", &self.tag_len)
                            .finish()
                    }
                }
                pub struct PacketKey {
                    pub ctx: EvpAeadCtx,
                    pub nonce: _rt::Vec<u8>,
                }
                #[automatically_derived]
                impl ::core::clone::Clone for PacketKey {
                    #[inline]
                    fn clone(&self) -> PacketKey {
                        PacketKey {
                            ctx: ::core::clone::Clone::clone(&self.ctx),
                            nonce: ::core::clone::Clone::clone(&self.nonce),
                        }
                    }
                }
                impl ::core::fmt::Debug for PacketKey {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        f.debug_struct("PacketKey")
                            .field("ctx", &self.ctx)
                            .field("nonce", &self.nonce)
                            .finish()
                    }
                }
                /// TODO: Nachfragen: aus ring-0.17.8/digest.rs L452
                /// union State {
                /// as64: [Wrapping<u64>; sha2::CHAINING_WORDS],
                /// as32: [Wrapping<u32>; sha2::CHAINING_WORDS],
                /// }
                pub enum DigestState {
                    A64(_rt::Vec<u64>),
                    A32(_rt::Vec<u32>),
                }
                #[automatically_derived]
                impl ::core::clone::Clone for DigestState {
                    #[inline]
                    fn clone(&self) -> DigestState {
                        match self {
                            DigestState::A64(__self_0) => {
                                DigestState::A64(::core::clone::Clone::clone(__self_0))
                            }
                            DigestState::A32(__self_0) => {
                                DigestState::A32(::core::clone::Clone::clone(__self_0))
                            }
                        }
                    }
                }
                impl ::core::fmt::Debug for DigestState {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        match self {
                            DigestState::A64(e) => {
                                f.debug_tuple("DigestState::A64").field(e).finish()
                            }
                            DigestState::A32(e) => {
                                f.debug_tuple("DigestState::A32").field(e).finish()
                            }
                        }
                    }
                }
                #[repr(u8)]
                pub enum DigestAlgorithmId {
                    Sha1,
                    Sha256,
                    Sha384,
                    Sha512,
                    Sha512256,
                }
                #[automatically_derived]
                impl ::core::clone::Clone for DigestAlgorithmId {
                    #[inline]
                    fn clone(&self) -> DigestAlgorithmId {
                        *self
                    }
                }
                #[automatically_derived]
                impl ::core::marker::Copy for DigestAlgorithmId {}
                #[automatically_derived]
                impl ::core::cmp::Eq for DigestAlgorithmId {
                    #[inline]
                    #[doc(hidden)]
                    #[coverage(off)]
                    fn assert_receiver_is_total_eq(&self) -> () {}
                }
                #[automatically_derived]
                impl ::core::cmp::Ord for DigestAlgorithmId {
                    #[inline]
                    fn cmp(&self, other: &DigestAlgorithmId) -> ::core::cmp::Ordering {
                        let __self_discr = ::core::intrinsics::discriminant_value(self);
                        let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                        ::core::cmp::Ord::cmp(&__self_discr, &__arg1_discr)
                    }
                }
                #[automatically_derived]
                impl ::core::marker::StructuralPartialEq for DigestAlgorithmId {}
                #[automatically_derived]
                impl ::core::cmp::PartialEq for DigestAlgorithmId {
                    #[inline]
                    fn eq(&self, other: &DigestAlgorithmId) -> bool {
                        let __self_discr = ::core::intrinsics::discriminant_value(self);
                        let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                        __self_discr == __arg1_discr
                    }
                }
                #[automatically_derived]
                impl ::core::cmp::PartialOrd for DigestAlgorithmId {
                    #[inline]
                    fn partial_cmp(
                        &self,
                        other: &DigestAlgorithmId,
                    ) -> ::core::option::Option<::core::cmp::Ordering> {
                        let __self_discr = ::core::intrinsics::discriminant_value(self);
                        let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                        ::core::cmp::PartialOrd::partial_cmp(
                            &__self_discr,
                            &__arg1_discr,
                        )
                    }
                }
                impl ::core::fmt::Debug for DigestAlgorithmId {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        match self {
                            DigestAlgorithmId::Sha1 => {
                                f.debug_tuple("DigestAlgorithmId::Sha1").finish()
                            }
                            DigestAlgorithmId::Sha256 => {
                                f.debug_tuple("DigestAlgorithmId::Sha256").finish()
                            }
                            DigestAlgorithmId::Sha384 => {
                                f.debug_tuple("DigestAlgorithmId::Sha384").finish()
                            }
                            DigestAlgorithmId::Sha512 => {
                                f.debug_tuple("DigestAlgorithmId::Sha512").finish()
                            }
                            DigestAlgorithmId::Sha512256 => {
                                f.debug_tuple("DigestAlgorithmId::Sha512256").finish()
                            }
                        }
                    }
                }
                impl DigestAlgorithmId {
                    #[doc(hidden)]
                    pub unsafe fn _lift(val: u8) -> DigestAlgorithmId {
                        if !true {
                            return ::core::mem::transmute(val);
                        }
                        match val {
                            0 => DigestAlgorithmId::Sha1,
                            1 => DigestAlgorithmId::Sha256,
                            2 => DigestAlgorithmId::Sha384,
                            3 => DigestAlgorithmId::Sha512,
                            4 => DigestAlgorithmId::Sha512256,
                            _ => {
                                ::core::panicking::panic_fmt(
                                    format_args!("invalid enum discriminant"),
                                );
                            }
                        }
                    }
                }
                pub struct DigestAlgorithm {
                    pub output_len: u32,
                    pub chaining_len: u32,
                    pub block_len: u32,
                    pub len_len: u32,
                    /// TODO: oh shit
                    /// block_data_order: unsafe extern "C" fn(state: &mut State, data: *const u8, num: c::size_t),
                    /// format_output: fn(input: State) -> Output,
                    pub initial_state: DigestState,
                    pub id: DigestAlgorithmId,
                }
                #[automatically_derived]
                impl ::core::clone::Clone for DigestAlgorithm {
                    #[inline]
                    fn clone(&self) -> DigestAlgorithm {
                        DigestAlgorithm {
                            output_len: ::core::clone::Clone::clone(&self.output_len),
                            chaining_len: ::core::clone::Clone::clone(
                                &self.chaining_len,
                            ),
                            block_len: ::core::clone::Clone::clone(&self.block_len),
                            len_len: ::core::clone::Clone::clone(&self.len_len),
                            initial_state: ::core::clone::Clone::clone(
                                &self.initial_state,
                            ),
                            id: ::core::clone::Clone::clone(&self.id),
                        }
                    }
                }
                impl ::core::fmt::Debug for DigestAlgorithm {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        f.debug_struct("DigestAlgorithm")
                            .field("output-len", &self.output_len)
                            .field("chaining-len", &self.chaining_len)
                            .field("block-len", &self.block_len)
                            .field("len-len", &self.len_len)
                            .field("initial-state", &self.initial_state)
                            .field("id", &self.id)
                            .finish()
                    }
                }
                pub struct DigestBlockContext {
                    pub state: DigestState,
                    pub completed_data_blocks: u64,
                    pub algorithm: DigestAlgorithm,
                }
                #[automatically_derived]
                impl ::core::clone::Clone for DigestBlockContext {
                    #[inline]
                    fn clone(&self) -> DigestBlockContext {
                        DigestBlockContext {
                            state: ::core::clone::Clone::clone(&self.state),
                            completed_data_blocks: ::core::clone::Clone::clone(
                                &self.completed_data_blocks,
                            ),
                            algorithm: ::core::clone::Clone::clone(&self.algorithm),
                        }
                    }
                }
                impl ::core::fmt::Debug for DigestBlockContext {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        f.debug_struct("DigestBlockContext")
                            .field("state", &self.state)
                            .field("completed-data-blocks", &self.completed_data_blocks)
                            .field("algorithm", &self.algorithm)
                            .finish()
                    }
                }
                pub struct HmacKey {
                    pub inner: DigestBlockContext,
                    pub outer: DigestBlockContext,
                }
                #[automatically_derived]
                impl ::core::clone::Clone for HmacKey {
                    #[inline]
                    fn clone(&self) -> HmacKey {
                        HmacKey {
                            inner: ::core::clone::Clone::clone(&self.inner),
                            outer: ::core::clone::Clone::clone(&self.outer),
                        }
                    }
                }
                impl ::core::fmt::Debug for HmacKey {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        f.debug_struct("HmacKey")
                            .field("inner", &self.inner)
                            .field("outer", &self.outer)
                            .finish()
                    }
                }
                pub struct PseudorandomKey {
                    pub key: HmacKey,
                }
                #[automatically_derived]
                impl ::core::clone::Clone for PseudorandomKey {
                    #[inline]
                    fn clone(&self) -> PseudorandomKey {
                        PseudorandomKey {
                            key: ::core::clone::Clone::clone(&self.key),
                        }
                    }
                }
                impl ::core::fmt::Debug for PseudorandomKey {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        f.debug_struct("PseudorandomKey")
                            .field("key", &self.key)
                            .finish()
                    }
                }
                #[allow(unused_unsafe, clippy::all)]
                pub fn hellostone() -> _rt::String {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 8],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: *mut u8) {
                            ::core::panicking::panic(
                                "internal error: entered unreachable code",
                            )
                        }
                        wit_import(ptr0);
                        let l1 = *ptr0.add(0).cast::<*mut u8>();
                        let l2 = *ptr0.add(4).cast::<usize>();
                        let len3 = l2;
                        let bytes3 = _rt::Vec::from_raw_parts(l1.cast(), len3, len3);
                        _rt::string_lift(bytes3)
                    }
                }
                #[allow(unused_unsafe, clippy::all)]
                pub fn packet_key_new(
                    alg: Algorithm,
                    key: &[u8],
                    iv: &[u8],
                    enc: u32,
                ) -> Result<PacketKey, QuicError> {
                    unsafe {
                        #[repr(align(8))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 48]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 48],
                        );
                        let vec0 = key;
                        let ptr0 = vec0.as_ptr().cast::<u8>();
                        let len0 = vec0.len();
                        let vec1 = iv;
                        let ptr1 = vec1.as_ptr().cast::<u8>();
                        let len1 = vec1.len();
                        let ptr2 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(
                            _: i32,
                            _: *mut u8,
                            _: usize,
                            _: *mut u8,
                            _: usize,
                            _: i32,
                            _: *mut u8,
                        ) {
                            ::core::panicking::panic(
                                "internal error: entered unreachable code",
                            )
                        }
                        wit_import(
                            alg.clone() as i32,
                            ptr0.cast_mut(),
                            len0,
                            ptr1.cast_mut(),
                            len1,
                            _rt::as_i32(&enc),
                            ptr2,
                        );
                        let l3 = i32::from(*ptr2.add(0).cast::<u8>());
                        match l3 {
                            0 => {
                                let e = {
                                    let l4 = *ptr2.add(8).cast::<i32>();
                                    let l5 = *ptr2.add(12).cast::<*mut u8>();
                                    let l6 = *ptr2.add(16).cast::<usize>();
                                    let len7 = l6;
                                    let l8 = *ptr2.add(24).cast::<i64>();
                                    let l9 = i32::from(*ptr2.add(32).cast::<u8>());
                                    let l10 = *ptr2.add(40).cast::<*mut u8>();
                                    let l11 = *ptr2.add(44).cast::<usize>();
                                    let len12 = l11;
                                    PacketKey {
                                        ctx: EvpAeadCtx {
                                            aead: l4 as u32,
                                            opaque: _rt::Vec::from_raw_parts(l5.cast(), len7, len7),
                                            alignment: l8 as u64,
                                            tag_len: l9 as u8,
                                        },
                                        nonce: _rt::Vec::from_raw_parts(l10.cast(), len12, len12),
                                    }
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l13 = i32::from(*ptr2.add(8).cast::<u8>());
                                    let v17 = match l13 {
                                        0 => QuicError::Done,
                                        1 => QuicError::Buffertooshort,
                                        2 => QuicError::Unknownversion,
                                        3 => QuicError::Invalidframe,
                                        4 => QuicError::Invalidpacket,
                                        5 => QuicError::Invalidstate,
                                        6 => {
                                            let e17 = {
                                                let l14 = *ptr2.add(16).cast::<i64>();
                                                l14 as u64
                                            };
                                            QuicError::Invalidstreamstate(e17)
                                        }
                                        7 => QuicError::Invalidtransportparam,
                                        8 => QuicError::Cryptofail,
                                        9 => QuicError::Tlsfail,
                                        10 => QuicError::Flowcontrol,
                                        11 => QuicError::Streamlimit,
                                        12 => {
                                            let e17 = {
                                                let l15 = *ptr2.add(16).cast::<i64>();
                                                l15 as u64
                                            };
                                            QuicError::Streamstopped(e17)
                                        }
                                        13 => {
                                            let e17 = {
                                                let l16 = *ptr2.add(16).cast::<i64>();
                                                l16 as u64
                                            };
                                            QuicError::Streamreset(e17)
                                        }
                                        14 => QuicError::Finalsize,
                                        15 => QuicError::Congestioncontrol,
                                        16 => QuicError::Idlimit,
                                        17 => QuicError::Outofidentifiers,
                                        18 => QuicError::Keyupdate,
                                        n => {
                                            if true {
                                                match (&n, &19) {
                                                    (left_val, right_val) => {
                                                        if !(*left_val == *right_val) {
                                                            let kind = ::core::panicking::AssertKind::Eq;
                                                            ::core::panicking::assert_failed(
                                                                kind,
                                                                &*left_val,
                                                                &*right_val,
                                                                ::core::option::Option::Some(
                                                                    format_args!("invalid enum discriminant"),
                                                                ),
                                                            );
                                                        }
                                                    }
                                                };
                                            }
                                            QuicError::Cryptobufferexceeded
                                        }
                                    };
                                    v17
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
                #[allow(unused_unsafe, clippy::all)]
                pub fn packet_key_from_secret_pkr(
                    aead: Algorithm,
                    secret_prk: &PseudorandomKey,
                    enc: u32,
                ) -> Result<PacketKey, QuicError> {
                    unsafe {
                        #[repr(align(8))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 128]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 128],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        *ptr0.add(0).cast::<u8>() = (aead.clone() as i32) as u8;
                        let PseudorandomKey { key: key1 } = secret_prk;
                        let HmacKey { inner: inner2, outer: outer2 } = key1;
                        let DigestBlockContext {
                            state: state3,
                            completed_data_blocks: completed_data_blocks3,
                            algorithm: algorithm3,
                        } = inner2;
                        match state3 {
                            DigestState::A64(e) => {
                                *ptr0.add(8).cast::<u8>() = (0i32) as u8;
                                let vec4 = e;
                                let ptr4 = vec4.as_ptr().cast::<u8>();
                                let len4 = vec4.len();
                                *ptr0.add(16).cast::<usize>() = len4;
                                *ptr0.add(12).cast::<*mut u8>() = ptr4.cast_mut();
                            }
                            DigestState::A32(e) => {
                                *ptr0.add(8).cast::<u8>() = (1i32) as u8;
                                let vec5 = e;
                                let ptr5 = vec5.as_ptr().cast::<u8>();
                                let len5 = vec5.len();
                                *ptr0.add(16).cast::<usize>() = len5;
                                *ptr0.add(12).cast::<*mut u8>() = ptr5.cast_mut();
                            }
                        }
                        *ptr0.add(24).cast::<i64>() = _rt::as_i64(
                            completed_data_blocks3,
                        );
                        let DigestAlgorithm {
                            output_len: output_len6,
                            chaining_len: chaining_len6,
                            block_len: block_len6,
                            len_len: len_len6,
                            initial_state: initial_state6,
                            id: id6,
                        } = algorithm3;
                        *ptr0.add(32).cast::<i32>() = _rt::as_i32(output_len6);
                        *ptr0.add(36).cast::<i32>() = _rt::as_i32(chaining_len6);
                        *ptr0.add(40).cast::<i32>() = _rt::as_i32(block_len6);
                        *ptr0.add(44).cast::<i32>() = _rt::as_i32(len_len6);
                        match initial_state6 {
                            DigestState::A64(e) => {
                                *ptr0.add(48).cast::<u8>() = (0i32) as u8;
                                let vec7 = e;
                                let ptr7 = vec7.as_ptr().cast::<u8>();
                                let len7 = vec7.len();
                                *ptr0.add(56).cast::<usize>() = len7;
                                *ptr0.add(52).cast::<*mut u8>() = ptr7.cast_mut();
                            }
                            DigestState::A32(e) => {
                                *ptr0.add(48).cast::<u8>() = (1i32) as u8;
                                let vec8 = e;
                                let ptr8 = vec8.as_ptr().cast::<u8>();
                                let len8 = vec8.len();
                                *ptr0.add(56).cast::<usize>() = len8;
                                *ptr0.add(52).cast::<*mut u8>() = ptr8.cast_mut();
                            }
                        }
                        *ptr0.add(60).cast::<u8>() = (id6.clone() as i32) as u8;
                        let DigestBlockContext {
                            state: state9,
                            completed_data_blocks: completed_data_blocks9,
                            algorithm: algorithm9,
                        } = outer2;
                        match state9 {
                            DigestState::A64(e) => {
                                *ptr0.add(64).cast::<u8>() = (0i32) as u8;
                                let vec10 = e;
                                let ptr10 = vec10.as_ptr().cast::<u8>();
                                let len10 = vec10.len();
                                *ptr0.add(72).cast::<usize>() = len10;
                                *ptr0.add(68).cast::<*mut u8>() = ptr10.cast_mut();
                            }
                            DigestState::A32(e) => {
                                *ptr0.add(64).cast::<u8>() = (1i32) as u8;
                                let vec11 = e;
                                let ptr11 = vec11.as_ptr().cast::<u8>();
                                let len11 = vec11.len();
                                *ptr0.add(72).cast::<usize>() = len11;
                                *ptr0.add(68).cast::<*mut u8>() = ptr11.cast_mut();
                            }
                        }
                        *ptr0.add(80).cast::<i64>() = _rt::as_i64(
                            completed_data_blocks9,
                        );
                        let DigestAlgorithm {
                            output_len: output_len12,
                            chaining_len: chaining_len12,
                            block_len: block_len12,
                            len_len: len_len12,
                            initial_state: initial_state12,
                            id: id12,
                        } = algorithm9;
                        *ptr0.add(88).cast::<i32>() = _rt::as_i32(output_len12);
                        *ptr0.add(92).cast::<i32>() = _rt::as_i32(chaining_len12);
                        *ptr0.add(96).cast::<i32>() = _rt::as_i32(block_len12);
                        *ptr0.add(100).cast::<i32>() = _rt::as_i32(len_len12);
                        match initial_state12 {
                            DigestState::A64(e) => {
                                *ptr0.add(104).cast::<u8>() = (0i32) as u8;
                                let vec13 = e;
                                let ptr13 = vec13.as_ptr().cast::<u8>();
                                let len13 = vec13.len();
                                *ptr0.add(112).cast::<usize>() = len13;
                                *ptr0.add(108).cast::<*mut u8>() = ptr13.cast_mut();
                            }
                            DigestState::A32(e) => {
                                *ptr0.add(104).cast::<u8>() = (1i32) as u8;
                                let vec14 = e;
                                let ptr14 = vec14.as_ptr().cast::<u8>();
                                let len14 = vec14.len();
                                *ptr0.add(112).cast::<usize>() = len14;
                                *ptr0.add(108).cast::<*mut u8>() = ptr14.cast_mut();
                            }
                        }
                        *ptr0.add(116).cast::<u8>() = (id12.clone() as i32) as u8;
                        *ptr0.add(120).cast::<i32>() = _rt::as_i32(&enc);
                        let ptr15 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: *mut u8, _: *mut u8) {
                            ::core::panicking::panic(
                                "internal error: entered unreachable code",
                            )
                        }
                        wit_import(ptr0, ptr15);
                        let l16 = i32::from(*ptr15.add(0).cast::<u8>());
                        match l16 {
                            0 => {
                                let e = {
                                    let l17 = *ptr15.add(8).cast::<i32>();
                                    let l18 = *ptr15.add(12).cast::<*mut u8>();
                                    let l19 = *ptr15.add(16).cast::<usize>();
                                    let len20 = l19;
                                    let l21 = *ptr15.add(24).cast::<i64>();
                                    let l22 = i32::from(*ptr15.add(32).cast::<u8>());
                                    let l23 = *ptr15.add(40).cast::<*mut u8>();
                                    let l24 = *ptr15.add(44).cast::<usize>();
                                    let len25 = l24;
                                    PacketKey {
                                        ctx: EvpAeadCtx {
                                            aead: l17 as u32,
                                            opaque: _rt::Vec::from_raw_parts(l18.cast(), len20, len20),
                                            alignment: l21 as u64,
                                            tag_len: l22 as u8,
                                        },
                                        nonce: _rt::Vec::from_raw_parts(l23.cast(), len25, len25),
                                    }
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l26 = i32::from(*ptr15.add(8).cast::<u8>());
                                    let v30 = match l26 {
                                        0 => QuicError::Done,
                                        1 => QuicError::Buffertooshort,
                                        2 => QuicError::Unknownversion,
                                        3 => QuicError::Invalidframe,
                                        4 => QuicError::Invalidpacket,
                                        5 => QuicError::Invalidstate,
                                        6 => {
                                            let e30 = {
                                                let l27 = *ptr15.add(16).cast::<i64>();
                                                l27 as u64
                                            };
                                            QuicError::Invalidstreamstate(e30)
                                        }
                                        7 => QuicError::Invalidtransportparam,
                                        8 => QuicError::Cryptofail,
                                        9 => QuicError::Tlsfail,
                                        10 => QuicError::Flowcontrol,
                                        11 => QuicError::Streamlimit,
                                        12 => {
                                            let e30 = {
                                                let l28 = *ptr15.add(16).cast::<i64>();
                                                l28 as u64
                                            };
                                            QuicError::Streamstopped(e30)
                                        }
                                        13 => {
                                            let e30 = {
                                                let l29 = *ptr15.add(16).cast::<i64>();
                                                l29 as u64
                                            };
                                            QuicError::Streamreset(e30)
                                        }
                                        14 => QuicError::Finalsize,
                                        15 => QuicError::Congestioncontrol,
                                        16 => QuicError::Idlimit,
                                        17 => QuicError::Outofidentifiers,
                                        18 => QuicError::Keyupdate,
                                        n => {
                                            if true {
                                                match (&n, &19) {
                                                    (left_val, right_val) => {
                                                        if !(*left_val == *right_val) {
                                                            let kind = ::core::panicking::AssertKind::Eq;
                                                            ::core::panicking::assert_failed(
                                                                kind,
                                                                &*left_val,
                                                                &*right_val,
                                                                ::core::option::Option::Some(
                                                                    format_args!("invalid enum discriminant"),
                                                                ),
                                                            );
                                                        }
                                                    }
                                                };
                                            }
                                            QuicError::Cryptobufferexceeded
                                        }
                                    };
                                    v30
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
        }
    }
    mod _rt {
        pub use alloc_crate::vec::Vec;
        pub use alloc_crate::string::String;
        pub unsafe fn string_lift(bytes: Vec<u8>) -> String {
            if true {
                String::from_utf8(bytes).unwrap()
            } else {
                String::from_utf8_unchecked(bytes)
            }
        }
        pub fn as_i32<T: AsI32>(t: T) -> i32 {
            t.as_i32()
        }
        pub trait AsI32 {
            fn as_i32(self) -> i32;
        }
        impl<'a, T: Copy + AsI32> AsI32 for &'a T {
            fn as_i32(self) -> i32 {
                (*self).as_i32()
            }
        }
        impl AsI32 for i32 {
            #[inline]
            fn as_i32(self) -> i32 {
                self as i32
            }
        }
        impl AsI32 for u32 {
            #[inline]
            fn as_i32(self) -> i32 {
                self as i32
            }
        }
        impl AsI32 for i16 {
            #[inline]
            fn as_i32(self) -> i32 {
                self as i32
            }
        }
        impl AsI32 for u16 {
            #[inline]
            fn as_i32(self) -> i32 {
                self as i32
            }
        }
        impl AsI32 for i8 {
            #[inline]
            fn as_i32(self) -> i32 {
                self as i32
            }
        }
        impl AsI32 for u8 {
            #[inline]
            fn as_i32(self) -> i32 {
                self as i32
            }
        }
        impl AsI32 for char {
            #[inline]
            fn as_i32(self) -> i32 {
                self as i32
            }
        }
        impl AsI32 for usize {
            #[inline]
            fn as_i32(self) -> i32 {
                self as i32
            }
        }
        pub unsafe fn invalid_enum_discriminant<T>() -> T {
            if true {
                {
                    ::core::panicking::panic_fmt(
                        format_args!("invalid enum discriminant"),
                    );
                }
            } else {
                core::hint::unreachable_unchecked()
            }
        }
        pub fn as_i64<T: AsI64>(t: T) -> i64 {
            t.as_i64()
        }
        pub trait AsI64 {
            fn as_i64(self) -> i64;
        }
        impl<'a, T: Copy + AsI64> AsI64 for &'a T {
            fn as_i64(self) -> i64 {
                (*self).as_i64()
            }
        }
        impl AsI64 for i64 {
            #[inline]
            fn as_i64(self) -> i64 {
                self as i64
            }
        }
        impl AsI64 for u64 {
            #[inline]
            fn as_i64(self) -> i64 {
                self as i64
            }
        }
        extern crate alloc as alloc_crate;
    }
    #[inline(never)]
    #[doc(hidden)]
    pub fn __link_custom_section_describing_imports() {
        wit_bindgen::rt::maybe_link_cabi_realloc();
    }
    const _: &[u8] = b"package cm:wasmquiche;\n\ninterface build-response {\n    type header = tuple<list<u8>, list<u8>>;\n\n    build-response: func(request: list<header>) -> tuple<list<header>, list<u8>>;\n}\n\nworld application {\n    export fake-copy: func(length: u32) -> list<u8>;\n}\n\nworld server {\n    import fake-copy: func(length: u32) -> list<u8>;\n}\n\n\n\ninterface boringssl {\n    hellostone: func() -> string;\n\n    type c-int = s32;\n    type c-void = u8;\n    type uintptr-t = u32;\n\n    record EVP-AEAD {\n        unused: c-void\n    }\n\n\n    // --\n\n    variant quic-error {\n        done,\n        buffertooshort,\n        unknownversion,\n        invalidframe,\n        invalidpacket,\n        invalidstate,\n        invalidstreamstate(u64),\n        invalidtransportparam,\n        cryptofail,\n        tlsfail,\n        flowcontrol,\n        streamlimit,\n        streamstopped(u64),\n        streamreset(u64),\n        finalsize,\n        congestioncontrol,\n        idlimit,\n        outofidentifiers,\n        keyupdate,\n        cryptobufferexceeded,\n    }\n\n    enum level {\n        initial,\n        zero-rtt,\n        handshake,\n        one-rtt,\n    }\n\n    enum algorithm {\n        aes128-gcm,\n        aes256-gcm,\n        chacha20-ploy1305,\n    }\n\n    record evp-aead-ctx {\n        aead: uintptr-t,\n        opaque: list<u8>,\n        alignment: u64,\n        tag-len: u8,\n    }\n\n    record packet-key {\n        ctx: evp-aead-ctx,\n        nonce: list<u8>,\n    }\n\n    record pseudorandom-key {\n        key: hmac-key,\n    }\n\n    record hmac-key {\n        inner: digest-block-context,\n        outer: digest-block-context,\n    }\n\n    record digest-block-context {\n        state: digest-state,\n        completed-data-blocks: u64,\n        algorithm: digest-algorithm,\n    }\n\n    // TODO: Nachfragen: aus ring-0.17.8/digest.rs L452\n    // union State {\n    //    as64: [Wrapping<u64>; sha2::CHAINING_WORDS],\n    //    as32: [Wrapping<u32>; sha2::CHAINING_WORDS],\n    // }\n    variant digest-state {\n        a64(list<u64>),\n        a32(list<u32>),\n    }\n\n    record digest-algorithm {\n        output-len: u32,\n        chaining-len: u32,\n        block-len: u32,\n        len-len: u32,\n\n        // TODO: oh shit\n        // block_data_order: unsafe extern \"C\" fn(state: &mut State, data: *const u8, num: c::size_t),\n        // format_output: fn(input: State) -> Output,\n\n        \n        initial-state: digest-state,\n        id: digest-algorithm-id,\n    }\n\n    enum digest-algorithm-id {\n        sha1,\n        sha256,\n        sha384,\n        sha512,\n        sha512256,\n    }\n\n    packet-key-new: func(alg: algorithm, key: list<u8>, iv: list<u8>, enc: u32) -> result<packet-key, quic-error>;\n\n    packet-key-from-secret-pkr: func(aead: algorithm, secret-prk: pseudorandom-key, enc: u32) -> result<packet-key, quic-error>;\n\n    \n\n    // EVP-aead-aes256-gcm: func() -> EVP-AEAD;\n}\n\nworld hellostones {\n    import boringssl;\n}\n\nworld hellostonesgreeter {\n    export boringssl;\n}\n";
    use std::convert::TryInto;
    use cm::wasmquiche::boringssl::hellostone;
    use super::*;
    #[repr(C)]
    struct EVP_AEAD_CTX {
        aead: libc::uintptr_t,
        opaque: [u8; 580],
        alignment: u64,
        tag_len: u8,
    }
    #[allow(unused_variables)]
    impl Open {
        pub fn open_with_u64_counter(
            &self,
            counter: u64,
            ad: &[u8],
            buf: &mut [u8],
        ) -> Result<usize> {
            ::core::panicking::panic("not implemented")
        }
    }
    #[allow(unused_variables)]
    impl Seal {
        pub fn seal_with_u64_counter(
            &self,
            counter: u64,
            ad: &[u8],
            buf: &mut [u8],
            in_len: usize,
            extra_in: Option<&[u8]>,
        ) -> Result<usize> {
            ::core::panicking::panic("not implemented")
        }
    }
    #[allow(unused_variables)]
    pub(crate) struct PacketKey {
        ctx: EVP_AEAD_CTX,
        nonce: Vec<u8>,
    }
    #[allow(unused_variables)]
    impl PacketKey {
        pub fn new(
            alg: Algorithm,
            key: Vec<u8>,
            iv: Vec<u8>,
            _enc: u32,
        ) -> Result<Self> {
            let alg = rust_alg_to_wit(alg);
            let intermediary_res = cm::wasmquiche::boringssl::packet_key_new(
                alg,
                &key,
                &iv,
                _enc,
            );
            match intermediary_res {
                Ok(pk) => Ok(wit_packet_key_to_rust(pk)),
                Err(e) => Err(wit_quic_error_to_rust(e)),
            }
        }
        pub fn from_secret_prk(
            aead: Algorithm,
            secret_prk: &hkdf::Prk,
            enc: u32,
        ) -> Result<Self> {
            let aead = rust_alg_to_wit(aead);
            let prk = rust_prk_to_wit(secret_prk);
            let intermediary_res = cm::wasmquiche::boringssl::packet_key_from_secret_pkr(
                aead,
                &prk,
                enc,
            );
            match intermediary_res {
                Ok(pk) => Ok(wit_packet_key_to_rust(pk)),
                Err(e) => Err(wit_quic_error_to_rust(e)),
            }
        }
    }
    fn wit_alg_to_rust(alg: cm::wasmquiche::boringssl::Algorithm) -> Algorithm {
        match alg {
            cm::wasmquiche::boringssl::Algorithm::Aes128Gcm => Algorithm::AES128_GCM,
            cm::wasmquiche::boringssl::Algorithm::Aes256Gcm => Algorithm::AES256_GCM,
            cm::wasmquiche::boringssl::Algorithm::Chacha20Ploy1305 => {
                Algorithm::ChaCha20_Poly1305
            }
        }
    }
    fn rust_alg_to_wit(alg: Algorithm) -> cm::wasmquiche::boringssl::Algorithm {
        match alg {
            Algorithm::AES128_GCM => cm::wasmquiche::boringssl::Algorithm::Aes128Gcm,
            Algorithm::AES256_GCM => cm::wasmquiche::boringssl::Algorithm::Aes256Gcm,
            Algorithm::ChaCha20_Poly1305 => {
                cm::wasmquiche::boringssl::Algorithm::Chacha20Ploy1305
            }
        }
    }
    fn wit_packet_key_to_rust(pk: cm::wasmquiche::boringssl::PacketKey) -> PacketKey {
        PacketKey {
            ctx: wit_evp_aead_ctx_to_rust(pk.ctx),
            nonce: pk.nonce,
        }
    }
    fn wit_evp_aead_ctx_to_rust(
        ctx: cm::wasmquiche::boringssl::EvpAeadCtx,
    ) -> EVP_AEAD_CTX {
        EVP_AEAD_CTX {
            aead: ctx.aead as usize,
            opaque: ctx.opaque.try_into().unwrap(),
            alignment: ctx.alignment,
            tag_len: ctx.tag_len,
        }
    }
    fn wit_quic_error_to_rust(err: cm::wasmquiche::boringssl::QuicError) -> Error {
        match err {
            cm::wasmquiche::boringssl::QuicError::Done => Error::Done,
            cm::wasmquiche::boringssl::QuicError::Buffertooshort => Error::BufferTooShort,
            cm::wasmquiche::boringssl::QuicError::Unknownversion => Error::UnknownVersion,
            cm::wasmquiche::boringssl::QuicError::Invalidframe => Error::InvalidFrame,
            cm::wasmquiche::boringssl::QuicError::Invalidpacket => Error::InvalidPacket,
            cm::wasmquiche::boringssl::QuicError::Invalidstate => Error::InvalidState,
            cm::wasmquiche::boringssl::QuicError::Invalidstreamstate(x) => {
                Error::InvalidStreamState(x)
            }
            cm::wasmquiche::boringssl::QuicError::Invalidtransportparam => {
                Error::InvalidTransportParam
            }
            cm::wasmquiche::boringssl::QuicError::Cryptofail => Error::CryptoFail,
            cm::wasmquiche::boringssl::QuicError::Tlsfail => Error::TlsFail,
            cm::wasmquiche::boringssl::QuicError::Flowcontrol => Error::FlowControl,
            cm::wasmquiche::boringssl::QuicError::Streamlimit => Error::StreamLimit,
            cm::wasmquiche::boringssl::QuicError::Streamstopped(x) => {
                Error::StreamStopped(x)
            }
            cm::wasmquiche::boringssl::QuicError::Streamreset(x) => Error::StreamReset(x),
            cm::wasmquiche::boringssl::QuicError::Finalsize => Error::FinalSize,
            cm::wasmquiche::boringssl::QuicError::Congestioncontrol => {
                Error::CongestionControl
            }
            cm::wasmquiche::boringssl::QuicError::Idlimit => Error::IdLimit,
            cm::wasmquiche::boringssl::QuicError::Outofidentifiers => {
                Error::OutOfIdentifiers
            }
            cm::wasmquiche::boringssl::QuicError::Keyupdate => Error::KeyUpdate,
            cm::wasmquiche::boringssl::QuicError::Cryptobufferexceeded => {
                Error::CryptoBufferExceeded
            }
        }
    }
    fn rust_prk_to_wit(
        secret_prk: &hkdf::Prk,
    ) -> cm::wasmquiche::boringssl::PseudorandomKey {
        cm::wasmquiche::boringssl::PseudorandomKey {
        }
    }
}
