///Module containing a contract's types and functions.
/**

```solidity
library IECDSAStakeRegistryTypes {
    struct Quorum { StrategyParams[] strategies; }
    struct StrategyParams { address strategy; uint96 multiplier; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod IECDSAStakeRegistryTypes {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /**```solidity
struct Quorum { StrategyParams[] strategies; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct Quorum {
        #[allow(missing_docs)]
        pub strategies: alloy::sol_types::private::Vec<
            <StrategyParams as alloy::sol_types::SolType>::RustType,
        >,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Array<StrategyParams>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Vec<
                <StrategyParams as alloy::sol_types::SolType>::RustType,
            >,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<Quorum> for UnderlyingRustTuple<'_> {
            fn from(value: Quorum) -> Self {
                (value.strategies,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for Quorum {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { strategies: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for Quorum {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for Quorum {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        StrategyParams,
                    > as alloy_sol_types::SolType>::tokenize(&self.strategies),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for Quorum {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for Quorum {
            const NAME: &'static str = "Quorum";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "Quorum(StrategyParams[] strategies)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                let mut components = alloy_sol_types::private::Vec::with_capacity(1);
                components
                    .push(
                        <StrategyParams as alloy_sol_types::SolStruct>::eip712_root_type(),
                    );
                components
                    .extend(
                        <StrategyParams as alloy_sol_types::SolStruct>::eip712_components(),
                    );
                components
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                <alloy::sol_types::sol_data::Array<
                    StrategyParams,
                > as alloy_sol_types::SolType>::eip712_data_word(&self.strategies)
                    .0
                    .to_vec()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for Quorum {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Array<
                        StrategyParams,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.strategies,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <alloy::sol_types::sol_data::Array<
                    StrategyParams,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.strategies,
                    out,
                );
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    rust,
                    &mut out,
                );
                alloy_sol_types::abi::token::WordToken(
                    alloy_sol_types::private::keccak256(out),
                )
            }
        }
    };
    /**```solidity
struct StrategyParams { address strategy; uint96 multiplier; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct StrategyParams {
        #[allow(missing_docs)]
        pub strategy: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub multiplier: alloy::sol_types::private::primitives::aliases::U96,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Uint<96>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
            alloy::sol_types::private::primitives::aliases::U96,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<StrategyParams> for UnderlyingRustTuple<'_> {
            fn from(value: StrategyParams) -> Self {
                (value.strategy, value.multiplier)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for StrategyParams {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    strategy: tuple.0,
                    multiplier: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for StrategyParams {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for StrategyParams {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.strategy,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        96,
                    > as alloy_sol_types::SolType>::tokenize(&self.multiplier),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for StrategyParams {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for StrategyParams {
            const NAME: &'static str = "StrategyParams";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "StrategyParams(address strategy,uint96 multiplier)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                alloy_sol_types::private::Vec::new()
            }
            #[inline]
            fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
                <Self as alloy_sol_types::SolStruct>::eip712_root_type()
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.strategy,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        96,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.multiplier)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for StrategyParams {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.strategy,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        96,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.multiplier,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.strategy,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    96,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.multiplier,
                    out,
                );
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    rust,
                    &mut out,
                );
                alloy_sol_types::abi::token::WordToken(
                    alloy_sol_types::private::keccak256(out),
                )
            }
        }
    };
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`IECDSAStakeRegistryTypes`](self) contract instance.

See the [wrapper's documentation](`IECDSAStakeRegistryTypesInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> IECDSAStakeRegistryTypesInstance<T, P, N> {
        IECDSAStakeRegistryTypesInstance::<T, P, N>::new(address, provider)
    }
    /**A [`IECDSAStakeRegistryTypes`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`IECDSAStakeRegistryTypes`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct IECDSAStakeRegistryTypesInstance<
        T,
        P,
        N = alloy_contract::private::Ethereum,
    > {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for IECDSAStakeRegistryTypesInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("IECDSAStakeRegistryTypesInstance")
                .field(&self.address)
                .finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > IECDSAStakeRegistryTypesInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`IECDSAStakeRegistryTypes`](self) contract instance.

See the [wrapper's documentation](`IECDSAStakeRegistryTypesInstance`) for more details.*/
        #[inline]
        pub const fn new(
            address: alloy_sol_types::private::Address,
            provider: P,
        ) -> Self {
            Self {
                address,
                provider,
                _network_transport: ::core::marker::PhantomData,
            }
        }
        /// Returns a reference to the address.
        #[inline]
        pub const fn address(&self) -> &alloy_sol_types::private::Address {
            &self.address
        }
        /// Sets the address.
        #[inline]
        pub fn set_address(&mut self, address: alloy_sol_types::private::Address) {
            self.address = address;
        }
        /// Sets the address and returns `self`.
        pub fn at(mut self, address: alloy_sol_types::private::Address) -> Self {
            self.set_address(address);
            self
        }
        /// Returns a reference to the provider.
        #[inline]
        pub const fn provider(&self) -> &P {
            &self.provider
        }
    }
    impl<T, P: ::core::clone::Clone, N> IECDSAStakeRegistryTypesInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> IECDSAStakeRegistryTypesInstance<T, P, N> {
            IECDSAStakeRegistryTypesInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network_transport: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > IECDSAStakeRegistryTypesInstance<T, P, N> {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<T, &P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > IECDSAStakeRegistryTypesInstance<T, P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<T, &P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
    }
}
///Module containing a contract's types and functions.
/**

```solidity
library ISignatureUtils {
    struct SignatureWithSaltAndExpiry { bytes signature; bytes32 salt; uint256 expiry; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod ISignatureUtils {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /**```solidity
struct SignatureWithSaltAndExpiry { bytes signature; bytes32 salt; uint256 expiry; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct SignatureWithSaltAndExpiry {
        #[allow(missing_docs)]
        pub signature: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub salt: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub expiry: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Bytes,
            alloy::sol_types::sol_data::FixedBytes<32>,
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Bytes,
            alloy::sol_types::private::FixedBytes<32>,
            alloy::sol_types::private::primitives::aliases::U256,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<SignatureWithSaltAndExpiry>
        for UnderlyingRustTuple<'_> {
            fn from(value: SignatureWithSaltAndExpiry) -> Self {
                (value.signature, value.salt, value.expiry)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for SignatureWithSaltAndExpiry {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    signature: tuple.0,
                    salt: tuple.1,
                    expiry: tuple.2,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for SignatureWithSaltAndExpiry {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self>
        for SignatureWithSaltAndExpiry {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.signature,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.salt),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.expiry),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for SignatureWithSaltAndExpiry {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for SignatureWithSaltAndExpiry {
            const NAME: &'static str = "SignatureWithSaltAndExpiry";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "SignatureWithSaltAndExpiry(bytes signature,bytes32 salt,uint256 expiry)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                alloy_sol_types::private::Vec::new()
            }
            #[inline]
            fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
                <Self as alloy_sol_types::SolStruct>::eip712_root_type()
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::eip712_data_word(
                            &self.signature,
                        )
                        .0,
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.salt)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.expiry)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for SignatureWithSaltAndExpiry {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.signature,
                    )
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.salt)
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.expiry,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.signature,
                    out,
                );
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.salt,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.expiry,
                    out,
                );
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    rust,
                    &mut out,
                );
                alloy_sol_types::abi::token::WordToken(
                    alloy_sol_types::private::keccak256(out),
                )
            }
        }
    };
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`ISignatureUtils`](self) contract instance.

See the [wrapper's documentation](`ISignatureUtilsInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> ISignatureUtilsInstance<T, P, N> {
        ISignatureUtilsInstance::<T, P, N>::new(address, provider)
    }
    /**A [`ISignatureUtils`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`ISignatureUtils`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct ISignatureUtilsInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for ISignatureUtilsInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("ISignatureUtilsInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > ISignatureUtilsInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`ISignatureUtils`](self) contract instance.

See the [wrapper's documentation](`ISignatureUtilsInstance`) for more details.*/
        #[inline]
        pub const fn new(
            address: alloy_sol_types::private::Address,
            provider: P,
        ) -> Self {
            Self {
                address,
                provider,
                _network_transport: ::core::marker::PhantomData,
            }
        }
        /// Returns a reference to the address.
        #[inline]
        pub const fn address(&self) -> &alloy_sol_types::private::Address {
            &self.address
        }
        /// Sets the address.
        #[inline]
        pub fn set_address(&mut self, address: alloy_sol_types::private::Address) {
            self.address = address;
        }
        /// Sets the address and returns `self`.
        pub fn at(mut self, address: alloy_sol_types::private::Address) -> Self {
            self.set_address(address);
            self
        }
        /// Returns a reference to the provider.
        #[inline]
        pub const fn provider(&self) -> &P {
            &self.provider
        }
    }
    impl<T, P: ::core::clone::Clone, N> ISignatureUtilsInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> ISignatureUtilsInstance<T, P, N> {
            ISignatureUtilsInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network_transport: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > ISignatureUtilsInstance<T, P, N> {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<T, &P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > ISignatureUtilsInstance<T, P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<T, &P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
    }
}
/**

Generated by the following Solidity interface...
```solidity
library IECDSAStakeRegistryTypes {
    struct Quorum {
        StrategyParams[] strategies;
    }
    struct StrategyParams {
        address strategy;
        uint96 multiplier;
    }
}

library ISignatureUtils {
    struct SignatureWithSaltAndExpiry {
        bytes signature;
        bytes32 salt;
        uint256 expiry;
    }
}

interface ECDSAStakeRegistry {
    error InsufficientSignedStake();
    error InsufficientWeight();
    error InvalidLength();
    error InvalidQuorum();
    error InvalidReferenceBlock();
    error InvalidSignature();
    error InvalidSignedWeight();
    error InvalidThreshold();
    error LengthMismatch();
    error MustUpdateAllOperators();
    error NotSorted();
    error OperatorAlreadyRegistered();
    error OperatorNotRegistered();

    event Initialized(uint8 version);
    event MinimumWeightUpdated(uint256 previous, uint256 current);
    event OperatorDeregistered(address indexed operator, address indexed avs);
    event OperatorRegistered(address indexed operator, address indexed avs);
    event OperatorWeightUpdated(address indexed operator, uint256 oldWeight, uint256 newWeight);
    event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);
    event QuorumUpdated(IECDSAStakeRegistryTypes.Quorum previous, IECDSAStakeRegistryTypes.Quorum current);
    event SigningKeyUpdate(address indexed operator, uint256 indexed updateBlock, address indexed newSigningKey, address oldSigningKey);
    event ThresholdWeightUpdated(uint256 thresholdWeight);
    event TotalWeightUpdated(uint256 oldTotalWeight, uint256 newTotalWeight);
    event UpdateMinimumWeight(uint256 oldMinimumWeight, uint256 newMinimumWeight);

    constructor(address _delegationManager);

    function deregisterOperator() external;
    function getLastCheckpointOperatorWeight(address operator) external view returns (uint256);
    function getLastCheckpointThresholdWeight() external view returns (uint256);
    function getLastCheckpointThresholdWeightAtBlock(uint32 blockNumber) external view returns (uint256);
    function getLastCheckpointTotalWeight() external view returns (uint256);
    function getLastCheckpointTotalWeightAtBlock(uint32 blockNumber) external view returns (uint256);
    function getLatestOperatorSigningKey(address operator) external view returns (address);
    function getOperatorSigningKeyAtBlock(address operator, uint256 blockNumber) external view returns (address);
    function getOperatorWeight(address operator) external view returns (uint256);
    function getOperatorWeightAtBlock(address operator, uint32 blockNumber) external view returns (uint256);
    function initialize(address _serviceManager, uint256 thresholdWeight, IECDSAStakeRegistryTypes.Quorum memory quorum) external;
    function isValidSignature(bytes32 digest, bytes memory _signatureData) external view returns (bytes4);
    function minimumWeight() external view returns (uint256);
    function operatorRegistered(address operator) external view returns (bool);
    function owner() external view returns (address);
    function quorum() external view returns (IECDSAStakeRegistryTypes.Quorum memory);
    function registerOperatorWithSignature(ISignatureUtils.SignatureWithSaltAndExpiry memory operatorSignature, address signingKey) external;
    function renounceOwnership() external;
    function transferOwnership(address newOwner) external;
    function updateMinimumWeight(uint256 newMinimumWeight, address[] memory operators) external;
    function updateOperatorSigningKey(address newSigningKey) external;
    function updateOperators(address[] memory operators) external;
    function updateOperatorsForQuorum(address[][] memory operatorsPerQuorum, bytes memory) external;
    function updateQuorumConfig(IECDSAStakeRegistryTypes.Quorum memory quorum, address[] memory operators) external;
    function updateStakeThreshold(uint256 thresholdWeight) external;
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "constructor",
    "inputs": [
      {
        "name": "_delegationManager",
        "type": "address",
        "internalType": "contract IDelegationManager"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "deregisterOperator",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "getLastCheckpointOperatorWeight",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getLastCheckpointThresholdWeight",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getLastCheckpointThresholdWeightAtBlock",
    "inputs": [
      {
        "name": "blockNumber",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getLastCheckpointTotalWeight",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getLastCheckpointTotalWeightAtBlock",
    "inputs": [
      {
        "name": "blockNumber",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getLatestOperatorSigningKey",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getOperatorSigningKeyAtBlock",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "blockNumber",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getOperatorWeight",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getOperatorWeightAtBlock",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "blockNumber",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "initialize",
    "inputs": [
      {
        "name": "_serviceManager",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "thresholdWeight",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "quorum",
        "type": "tuple",
        "internalType": "struct IECDSAStakeRegistryTypes.Quorum",
        "components": [
          {
            "name": "strategies",
            "type": "tuple[]",
            "internalType": "struct IECDSAStakeRegistryTypes.StrategyParams[]",
            "components": [
              {
                "name": "strategy",
                "type": "address",
                "internalType": "contract IStrategy"
              },
              {
                "name": "multiplier",
                "type": "uint96",
                "internalType": "uint96"
              }
            ]
          }
        ]
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "isValidSignature",
    "inputs": [
      {
        "name": "digest",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "_signatureData",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bytes4",
        "internalType": "bytes4"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "minimumWeight",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "operatorRegistered",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "owner",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "quorum",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "tuple",
        "internalType": "struct IECDSAStakeRegistryTypes.Quorum",
        "components": [
          {
            "name": "strategies",
            "type": "tuple[]",
            "internalType": "struct IECDSAStakeRegistryTypes.StrategyParams[]",
            "components": [
              {
                "name": "strategy",
                "type": "address",
                "internalType": "contract IStrategy"
              },
              {
                "name": "multiplier",
                "type": "uint96",
                "internalType": "uint96"
              }
            ]
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "registerOperatorWithSignature",
    "inputs": [
      {
        "name": "operatorSignature",
        "type": "tuple",
        "internalType": "struct ISignatureUtils.SignatureWithSaltAndExpiry",
        "components": [
          {
            "name": "signature",
            "type": "bytes",
            "internalType": "bytes"
          },
          {
            "name": "salt",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "expiry",
            "type": "uint256",
            "internalType": "uint256"
          }
        ]
      },
      {
        "name": "signingKey",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "renounceOwnership",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "transferOwnership",
    "inputs": [
      {
        "name": "newOwner",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "updateMinimumWeight",
    "inputs": [
      {
        "name": "newMinimumWeight",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "operators",
        "type": "address[]",
        "internalType": "address[]"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "updateOperatorSigningKey",
    "inputs": [
      {
        "name": "newSigningKey",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "updateOperators",
    "inputs": [
      {
        "name": "operators",
        "type": "address[]",
        "internalType": "address[]"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "updateOperatorsForQuorum",
    "inputs": [
      {
        "name": "operatorsPerQuorum",
        "type": "address[][]",
        "internalType": "address[][]"
      },
      {
        "name": "",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "updateQuorumConfig",
    "inputs": [
      {
        "name": "quorum",
        "type": "tuple",
        "internalType": "struct IECDSAStakeRegistryTypes.Quorum",
        "components": [
          {
            "name": "strategies",
            "type": "tuple[]",
            "internalType": "struct IECDSAStakeRegistryTypes.StrategyParams[]",
            "components": [
              {
                "name": "strategy",
                "type": "address",
                "internalType": "contract IStrategy"
              },
              {
                "name": "multiplier",
                "type": "uint96",
                "internalType": "uint96"
              }
            ]
          }
        ]
      },
      {
        "name": "operators",
        "type": "address[]",
        "internalType": "address[]"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "updateStakeThreshold",
    "inputs": [
      {
        "name": "thresholdWeight",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "event",
    "name": "Initialized",
    "inputs": [
      {
        "name": "version",
        "type": "uint8",
        "indexed": false,
        "internalType": "uint8"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "MinimumWeightUpdated",
    "inputs": [
      {
        "name": "previous",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "current",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "OperatorDeregistered",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "avs",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "OperatorRegistered",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "avs",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "OperatorWeightUpdated",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "oldWeight",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "newWeight",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "OwnershipTransferred",
    "inputs": [
      {
        "name": "previousOwner",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "newOwner",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "QuorumUpdated",
    "inputs": [
      {
        "name": "previous",
        "type": "tuple",
        "indexed": false,
        "internalType": "struct IECDSAStakeRegistryTypes.Quorum",
        "components": [
          {
            "name": "strategies",
            "type": "tuple[]",
            "internalType": "struct IECDSAStakeRegistryTypes.StrategyParams[]",
            "components": [
              {
                "name": "strategy",
                "type": "address",
                "internalType": "contract IStrategy"
              },
              {
                "name": "multiplier",
                "type": "uint96",
                "internalType": "uint96"
              }
            ]
          }
        ]
      },
      {
        "name": "current",
        "type": "tuple",
        "indexed": false,
        "internalType": "struct IECDSAStakeRegistryTypes.Quorum",
        "components": [
          {
            "name": "strategies",
            "type": "tuple[]",
            "internalType": "struct IECDSAStakeRegistryTypes.StrategyParams[]",
            "components": [
              {
                "name": "strategy",
                "type": "address",
                "internalType": "contract IStrategy"
              },
              {
                "name": "multiplier",
                "type": "uint96",
                "internalType": "uint96"
              }
            ]
          }
        ]
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "SigningKeyUpdate",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "updateBlock",
        "type": "uint256",
        "indexed": true,
        "internalType": "uint256"
      },
      {
        "name": "newSigningKey",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "oldSigningKey",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "ThresholdWeightUpdated",
    "inputs": [
      {
        "name": "thresholdWeight",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "TotalWeightUpdated",
    "inputs": [
      {
        "name": "oldTotalWeight",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "newTotalWeight",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "UpdateMinimumWeight",
    "inputs": [
      {
        "name": "oldMinimumWeight",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "newMinimumWeight",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "error",
    "name": "InsufficientSignedStake",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InsufficientWeight",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidLength",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidQuorum",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidReferenceBlock",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidSignature",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidSignedWeight",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidThreshold",
    "inputs": []
  },
  {
    "type": "error",
    "name": "LengthMismatch",
    "inputs": []
  },
  {
    "type": "error",
    "name": "MustUpdateAllOperators",
    "inputs": []
  },
  {
    "type": "error",
    "name": "NotSorted",
    "inputs": []
  },
  {
    "type": "error",
    "name": "OperatorAlreadyRegistered",
    "inputs": []
  },
  {
    "type": "error",
    "name": "OperatorNotRegistered",
    "inputs": []
  }
]
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod ECDSAStakeRegistry {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x60a034606657601f6123fd38819003918201601f19168301916001600160401b03831184841017606a57808492602094604052833981010312606657516001600160a01b038116810360665760805260405161237e908161007f82396080518161133b0152f35b5f80fd5b634e487b7160e01b5f52604160045260245ffdfe60806040526004361015610011575f80fd5b5f5f3560e01c8062cf2ab514610f0a5780630dba339414610ee15780631626ba7e14610bd75780631703a01814610b875780631e4cd85e14610b5e578063314f3a4914610b445780633b242e4a14610b0d5780633d5611f61461092457806340bf2fb7146109065780635140a548146107fa5780635e1042e8146107b45780635ef533291461078f578063696255be14610714578063715018a6146106b7578063743c31f41461066f578063857dc1901461055a5780638da5cb5b14610531578063955f2d90146104e457806398ec1ac9146104b8578063a2ce5fd11461046d578063ab118995146102d8578063b933fa7414610270578063dec5d1f6146101fe578063ec7fbb31146101bf5763f2fde38b1461012c575f80fd5b346101bc5760203660031901126101bc57610145610fad565b61014d611761565b6001600160a01b038116156101685761016590611971565b80f35b60405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b6064820152608490fd5b80fd5b50346101bc5760203660031901126101bc5760209060ff906040906001600160a01b036101ea610fad565b168152606e84522054166040519015158152f35b50346101bc5760403660031901126101bc576004356001600160401b03811161026c5761022f9036906004016110fe565b602435906001600160401b03821161026857610263610255610165933690600401610fc3565b9161025e611761565b611a9e565b6114b4565b8280fd5b5080fd5b50346101bc57806003193601126101bc57606c5490816102a257602091505b6040516001600160e01b03919091168152f35b5f1982019182116102c457506102bb602091606c611503565b5054811c61028f565b634e487b7160e01b81526011600452602490fd5b50346101bc5760603660031901126101bc576102f2610fad565b6044356001600160401b038111610268576103119036906004016110fe565b82549060ff8260081c161592838094610460575b8015610449575b156103ed5760ff198316600117855561037c92846103dc575b5061035660ff865460081c16611a3e565b60018060a01b03166001600160601b0360a01b606854161760685561025e6024356117b9565b61039560ff835460081c1661039081611a3e565b611a3e565b61039e33611971565b6103a55780f35b61ff001981541681557f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb3847402498602060405160018152a180f35b61ffff19166101011785555f610345565b60405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b6064820152608490fd5b50303b15801561032c5750600160ff84161461032c565b50600160ff841610610325565b50346101bc5760203660031901126101bc576020906001600160a01b03610492610fad565b168152606a8252604090206001600160a01b03906104af9061172f565b16604051908152f35b50346101bc5760203660031901126101bc5760206104dc6104d7610fad565b61127b565b604051908152f35b50346101bc5760403660031901126101bc576104fe610fad565b6024359063ffffffff8216809203610268576001600160a01b03168252606d602090815260409092206104dc9190611692565b50346101bc57806003193601126101bc576033546040516001600160a01b039091168152602090f35b50346101bc57806003193601126101bc57338152606e60205260ff6040822054161561066057606554801561064c575f1901606555338152606e6020526040812060ff1981541690556105b46105af33611c03565b611e00565b505060685481906001600160a01b0316803b15610649578180916024604051809481936351b27a6d60e11b83523360048401525af1801561063e57610629575b506068546001600160a01b0316337f31e0adfec71bccee37b6e83a90c2fedb17d8f1693fee863c4771e7bfe2aed5808380a380f35b8161063391610f75565b6101bc57805f6105f4565b6040513d84823e3d90fd5b50fd5b634e487b7160e01b82526011600452602482fd5b6325ec6c1f60e01b8152600490fd5b50346101bc5760203660031901126101bc57610689610fad565b338252606e60205260ff604083205416156106a85761016590336119b9565b6325ec6c1f60e01b8252600482fd5b50346101bc57806003193601126101bc576106d0611761565b603380546001600160a01b0319811690915581906001600160a01b03167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e08280a380f35b50346101bc5760403660031901126101bc576024356004356001600160401b038211610268577f713ca53b88d6eb63f5b1854cb8cbdd736ec51eda225e46791aa9298b0160648f604061076e610165943690600401610fc3565b92610777611761565b606754908060675582519182526020820152a16114b4565b50346101bc5760203660031901126101bc576107a9611761565b6101656004356117b9565b50346101bc5760403660031901126101bc576020906001600160a01b036107d9610fad565b168152606a8252604090206001600160a01b03906104af9060243590611692565b50346101bc5760403660031901126101bc576004356001600160401b03811161026c573660238201121561026c5780600401359061083782610f96565b906108456040519283610f75565b828252602082019260051b8101602401833682116109025760248301905b8282106108cf57505050506024356001600160401b0381116108cb5761088d90369060040161105f565b5051156108b757518051606554036108a857610165906114b4565b63169efb5b60e11b8252600482fd5b634e487b7160e01b82526032600452602482fd5b8380fd5b81356001600160401b0381116108fe576020916108f3839260243691890101610fc3565b815201910190610863565b8780fd5b8580fd5b50346101bc57806003193601126101bc576020606754604051908152f35b5034610ad2576040366003190112610ad2576004356001600160401b038111610ad25760606003198236030112610ad257604051606081018181106001600160401b03821117610af95760405281600401356001600160401b038111610ad257610994906004369185010161105f565b8152602480830135602083019081526044909301356040830190815290356001600160a01b0381168103610ad257335f52606e60205260ff60405f205416610aea57606554905f198214610ad6576001610a179201606555335f52606e60205260405f20600160ff19825416179055610a0f6105af33611c03565b5050336119b9565b6068546001600160a01b0316803b15610ad257610a635f809460405196879586948593639926ee7d60e01b855233600486015260406024860152516060604486015260a485019061173d565b9151606484015251608483015203925af18015610ac757610ab4575b506068546001600160a01b0316337fa453db612af59e5521d6ab9284dc3e2d06af286eb1b1b7b771fce4716c19f2c18380a380f35b610ac091505f90610f75565b5f5f610a7f565b6040513d5f823e3d90fd5b5f80fd5b634e487b7160e01b5f52601160045260245ffd5b6342ee68b560e01b5f5260045ffd5b634e487b7160e01b5f52604160045260245ffd5b34610ad2576020366003190112610ad2576001600160a01b03610b2e610fad565b165f52606d60205260206104dc60405f2061172f565b34610ad2575f366003190112610ad25760206104dc611722565b34610ad2576020366003190112610ad25760206104dc63ffffffff610b81611031565b16611601565b34610ad2575f366003190112610ad2576060604051610ba581610f3f565b52610bd3604051610bb581610f3f565b610bbd6111cf565b81526040519182916020835260208301906110a5565b0390f35b34610ad2576040366003190112610ad2576004356024356001600160401b038111610ad257610c0a90369060040161105f565b8051810160608260208301920312610ad25760208201516001600160401b038111610ad25782019181603f84011215610ad2576020830151610c4b81610f96565b93610c596040519586610f75565b8185526020808087019360051b8301010190848211610ad257604001915b818310610ec15750505060408101516001600160401b038111610ad25781019180603f84011215610ad257602083015192610cb184610f96565b93610cbf6040519586610f75565b8085526020808087019260051b8401010191838311610ad25760408101915b838310610e575788888860608901519163ffffffff8316809303610ad2578051915f5f9582518503610e48578415610e395743861015945f949093925b848610610d8557888888610d7657610d3281611518565b8211610d6757610d4190611601565b11610d5857604051630b135d3f60e11b8152602090f35b63e121632f60e01b5f5260045ffd5b634b05a0f760e11b5f5260045ffd5b63e64f180f60e01b5f5260045ffd5b949793949293926001600160a01b03610d9e8a84611246565b511696610d7657865f52606a6020528660018060a01b03610dc28a60405f20611692565b16916001600160a01b03161015610e2a57610de99083610de28b88611246565b5191612091565b15610e1b576001610e1187945f985f52606d602052610e0b8a60405f20611692565b9061126e565b9801949392610d1b565b638baa579f60e01b5f5260045ffd5b63ba50f91160e01b5f5260045ffd5b63251f56a160e21b5f5260045ffd5b631fec674760e31b5f5260045ffd5b82516001600160401b038111610ad25760209083010185603f82011215610ad257602081015191610e8783611044565b610e946040519182610f75565b8381526040838501018810610ad2575f602085819660408397018386015e83010152815201920191610cde565b82516001600160a01b0381168103610ad257815260209283019201610c77565b34610ad2576020366003190112610ad25760206104dc63ffffffff610f04611031565b16611518565b34610ad2576020366003190112610ad2576004356001600160401b038111610ad257610263610f3d913690600401610fc3565b005b602081019081106001600160401b03821117610af957604052565b604081019081106001600160401b03821117610af957604052565b90601f801991011681019081106001600160401b03821117610af957604052565b6001600160401b038111610af95760051b60200190565b600435906001600160a01b0382168203610ad257565b9080601f83011215610ad257813590610fdb82610f96565b92610fe96040519485610f75565b82845260208085019360051b820101918211610ad257602001915b8183106110115750505090565b82356001600160a01b0381168103610ad257815260209283019201611004565b6004359063ffffffff82168203610ad257565b6001600160401b038111610af957601f01601f191660200190565b81601f82011215610ad25780359061107682611044565b926110846040519485610f75565b82845260208383010111610ad257815f926020809301838601378301015290565b602060408184019251938281528451809452019201905f5b8181106110ca5750505090565b825180516001600160a01b031685526020908101516001600160601b031681860152604090940193909201916001016110bd565b9190602083820312610ad25760405161111681610f3f565b80938035906001600160401b038211610ad2570182601f82011215610ad25780359061114182610f96565b9361114f6040519586610f75565b82855260208086019360061b83010191818311610ad257602001925b828410611179575050505052565b604084830312610ad2576040519061119082610f5a565b84356001600160a01b0381168103610ad25782526020850135906001600160601b0382168203610ad2578260209283604095015281520193019261116b565b606654906111dc82610f96565b916111ea6040519384610f75565b80835260665f9081525f5160206123295f395f51905f52602085015b8383106112135750505050565b60016020819260405161122581610f5a565b8554848060a01b038116825260a01c83820152815201920192019190611206565b805182101561125a5760209160051b010190565b634e487b7160e01b5f52603260045260245ffd5b91908201809211610ad657565b6112836111cf565b805190925f6112aa61129484610f96565b936112a26040519586610f75565b808552610f96565b6020840190601f19013682375f5b86518110156112ea576001906001600160a01b036112d6828a611246565b5151166112e38288611246565b52016112b8565b5060408051639004134760e01b81526001600160a01b0390961660048701526024860152925160448501819052919390929091829160648301915f5b81811061147757505f939283900391508290507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa908115610ac7575f916113e7575b505f915b84518310156113c95761138a8383611246565b51906001600160601b0360206113a08689611246565b5101511691828102928184041490151715610ad6576001916113c19161126e565b920191611377565b915050612710919250046067548110155f146113e25790565b505f90565b90503d805f833e6113f88183610f75565b810190602081830312610ad2578051906001600160401b038211610ad257019080601f83011215610ad257815161142e81610f96565b9261143c6040519485610f75565b81845260208085019260051b820101928311610ad257602001905b828210611467575050505f611373565b8151815260209182019101611457565b82516001600160a01b0316845285945060209384019390920191600101611326565b9190915f8382019384129112908015821691151617610ad657565b905f805b83518210156114f2576001906114ea906114e46001600160a01b036114dd8689611246565b5116611c03565b90611499565b9101906114b8565b90506114ff919250611e00565b5050565b805482101561125a575f5260205f2001905f90565b438110156115bd57606b54905f905b8282106115645750508061154457505f5b6001600160e01b031690565b5f198101908111610ad65761155a90606b611503565b505460201c611538565b909161157781841860011c82851661126e565b908263ffffffff61158984606b611503565b505416111561159b5750915b90611527565b9250600181018091111561159557634e487b7160e01b5f52601160045260245ffd5b606460405162461bcd60e51b815260206004820152602060248201527f436865636b706f696e74733a20626c6f636b206e6f7420796574206d696e65646044820152fd5b438110156115bd57606c54905f905b8282106116395750508061162357505f90565b5f198101908111610ad65761155a90606c611503565b909161164c81841860011c82851661126e565b908263ffffffff61165e84606c611503565b50541611156116705750915b90611610565b9250600181018091111561166a57634e487b7160e01b5f52601160045260245ffd5b90438110156115bd578154905f905b8282106116ca575050806116b657505f919050565b5f198101908111610ad65761155a91611503565b90916116dd81841860011c82851661126e565b908263ffffffff6116ee8488611503565b50541611156117005750915b906116a1565b925060018101809111156116fa57634e487b7160e01b5f52601160045260245ffd5b606b548061154457505f90565b8054806116b657505f919050565b805180835260209291819084018484015e5f828201840152601f01601f1916010190565b6033546001600160a01b0316330361177557565b606460405162461bcd60e51b815260206004820152602060248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152fd5b606c54906117c7606c61172f565b908215158061194b575b1561183f576117df816121a9565b915f198401938411610ad65761183560209361181d7f9324f7e5a7c0288808a634ccde44b8e979676474b22e29ee9dd569b55e791a4b96606c611503565b509063ffffffff82549181199060201b169116179055565b50604051908152a1565b63ffffffff43116118f757611853816121a9565b916040519361186185610f5a565b4363ffffffff1685526001600160e01b039093166020850190815292600160401b811015610af95780600161189b9201606c55606c611503565b9490946118e45751925163ffffffff90931663ffffffff19602094851b16179093557f9324f7e5a7c0288808a634ccde44b8e979676474b22e29ee9dd569b55e791a4b92611835565b634e487b7160e01b5f525f60045260245ffd5b60405162461bcd60e51b815260206004820152602660248201527f53616665436173743a2076616c756520646f65736e27742066697420696e203360448201526532206269747360d01b6064820152608490fd5b505f198301838111610ad65761196763ffffffff91606c611503565b50541643146117d1565b603380546001600160a01b039283166001600160a01b0319821681179092559091167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e05f80a3565b6001600160a01b039081165f818152606a60205260409020909291906119de9061172f565b6001600160a01b0390921692911690828214611a3957805f52606a602052611a098360405f20611f2e565b50506040519182527fd061168252f441733658f09e4d8f5b2d998ed4ef24a2bbfd6ceca52ea131500260204393a4565b505050565b15611a4557565b60405162461bcd60e51b815260206004820152602b60248201527f496e697469616c697a61626c653a20636f6e7472616374206973206e6f74206960448201526a6e697469616c697a696e6760a81b6064820152608490fd5b611aa781612017565b15611bdc5760405191611ab983610f3f565b611ac16111cf565b83526066545f60665580611ba2575b505f5b82518051821015611b4f5781611ae891611246565b519060665491600160401b831015610af9576001830160665560665483101561125a5760665f5280516020919091015160a01b6001600160a01b0319166001600160a01b0391909116175f5160206123295f395f51905f5290920191909155600101611ad3565b505091611b8f90611b9d7f23aad4e61744ece164130aa415c1616e80136b0f0770e56589438b90b269265e936040519384936040855260408501906110a5565b9083820360208501526110a5565b0390a1565b60665f525f5160206123295f395f51905f52015f5160206123295f395f51905f525b818110611bd15750611ad0565b5f8155600101611bc4565b63d173577960e01b5f5260045ffd5b81810392915f138015828513169184121617610ad657565b6001600160a01b0381165f818152606d602052604081209092908390611c289061172f565b8094845f52606e60205260ff60405f205416155f14611da257611c4c929350611beb565b928315611d9c57825f52606d60205260405f2090815492611c6c8361172f565b84151580611d77575b15611cd057611c835f6121a9565b945f198101908111610ad6577f88770dc862e47a7ed586907857eb1b75e4c5ffc8b707c7ee10eb74d6885fe5949561181d611cc092604097611503565b505b82519182526020820152a290565b63ffffffff43116118f757611ce45f6121a9565b9360405195611cf287610f5a565b4363ffffffff1687526001600160e01b039095166020870190815294600160401b811015610af957611d2991600182018155611503565b9590956118e45751935160201b63ffffffff191663ffffffff94909416939093179093557f88770dc862e47a7ed586907857eb1b75e4c5ffc8b707c7ee10eb74d6885fe59492604092611cc0565b505f198501858111610ad657611d9263ffffffff9186611503565b5054164314611c75565b50505090565b505050611dae9061127b565b91611db98184611beb565b928315611d9c577f88770dc862e47a7ed586907857eb1b75e4c5ffc8b707c7ee10eb74d6885fe59491604091845f52606d602052611df981845f20611f2e565b5050611cc2565b90611e13611e0c611722565b9283611499565b90606b54611e21606b61172f565b9080151580611f08575b15611e8457611e39846121a9565b5f198201918211610ad65761181d611e5292606b611503565b507f86dcf86b12dfeedea74ae9300dbdaa193bcce5809369c8177ea2f4eaaa65729b60408051858152846020820152a1565b63ffffffff43116118f757611e98846121a9565b60405191611ea583610f5a565b4363ffffffff1683526001600160e01b039091166020830190815290600160401b811015610af957806001611edf9201606b55606b611503565b9190916118e4579151915160201b63ffffffff191663ffffffff92909216919091179055611e52565b505f198101818111610ad657611f2463ffffffff91606b611503565b5054164314611e2b565b9190918054611f3c8261172f565b9181151580611ff2575b15611f7157611f54856121a9565b905f198301928311610ad657611f6d9261181d91611503565b9190565b63ffffffff43116118f757611f85856121a9565b9060405192611f9384610f5a565b4363ffffffff1684526001600160e01b039092166020840190815291600160401b811015610af957611fca91600182018155611503565b9190916118e4579151915160201b63ffffffff191663ffffffff929092169190911790559190565b505f198201828111610ad65761200d63ffffffff9183611503565b5054164314611f46565b51905f80805b845182101561207a576001600160a01b036120388387611246565b515116906001600160a01b0316811115610e2a57612071600191936001600160601b036020612067868a611246565b510151169061126e565b9101909161201d565b5091925061271014905061208c575f90565b600190565b91909161209e8284612218565b60058110156121955715908161217f575b50612177575f926120e96120f785946040519283916020830195630b135d3f60e11b8752602484015260406044840152606483019061173d565b03601f198101835282610f75565b51915afa3d15612170573d61210b81611044565b906121196040519283610f75565b81523d5f602083013e5b81612164575b81612132575090565b9050602081805181010312610ad257602001516001600160e01b0319811690819003610ad257630b135d3f60e11b1490565b80516020149150612129565b6060612123565b505050600190565b6001600160a01b0383811691161490505f6120af565b634e487b7160e01b5f52602160045260245ffd5b6001600160e01b0381116121c3576001600160e01b031690565b60405162461bcd60e51b815260206004820152602760248201527f53616665436173743a2076616c756520646f65736e27742066697420696e20326044820152663234206269747360c81b6064820152608490fd5b81516041810361224457509061224091602082015190606060408401519301515f1a90612286565b9091565b60400361227d5760406020830151920151918260ff1c91601b8301809311610ad657612240936001600160ff1b03169260ff1690612286565b50505f90600290565b907f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a0841161231d5760ff1690601b82141580612312575b612307576020935f93608093604051938452868401526040830152606082015282805260015afa15610ac7575f516001600160a01b038116156122ff57905f90565b505f90600190565b505050505f90600490565b50601c8214156122bd565b505050505f9060039056fe46501879b8ca8525e8c2fd519e2fbfcfa2ebea26501294aa02cbfcfb12e94354a2646970667358221220d47de43e6e0e951a74532099040e816f8134af5294bff6965b453b994451762964736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\xA04`fW`\x1Fa#\xFD8\x81\x90\x03\x91\x82\x01`\x1F\x19\x16\x83\x01\x91`\x01`\x01`@\x1B\x03\x83\x11\x84\x84\x10\x17`jW\x80\x84\x92` \x94`@R\x839\x81\x01\x03\x12`fWQ`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03`fW`\x80R`@Qa#~\x90\x81a\0\x7F\x829`\x80Q\x81a\x13;\x01R\xF3[_\x80\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD\xFE`\x80`@R`\x046\x10\x15a\0\x11W_\x80\xFD[__5`\xE0\x1C\x80b\xCF*\xB5\x14a\x0F\nW\x80c\r\xBA3\x94\x14a\x0E\xE1W\x80c\x16&\xBA~\x14a\x0B\xD7W\x80c\x17\x03\xA0\x18\x14a\x0B\x87W\x80c\x1EL\xD8^\x14a\x0B^W\x80c1O:I\x14a\x0BDW\x80c;$.J\x14a\x0B\rW\x80c=V\x11\xF6\x14a\t$W\x80c@\xBF/\xB7\x14a\t\x06W\x80cQ@\xA5H\x14a\x07\xFAW\x80c^\x10B\xE8\x14a\x07\xB4W\x80c^\xF53)\x14a\x07\x8FW\x80cibU\xBE\x14a\x07\x14W\x80cqP\x18\xA6\x14a\x06\xB7W\x80ct<1\xF4\x14a\x06oW\x80c\x85}\xC1\x90\x14a\x05ZW\x80c\x8D\xA5\xCB[\x14a\x051W\x80c\x95_-\x90\x14a\x04\xE4W\x80c\x98\xEC\x1A\xC9\x14a\x04\xB8W\x80c\xA2\xCE_\xD1\x14a\x04mW\x80c\xAB\x11\x89\x95\x14a\x02\xD8W\x80c\xB93\xFAt\x14a\x02pW\x80c\xDE\xC5\xD1\xF6\x14a\x01\xFEW\x80c\xEC\x7F\xBB1\x14a\x01\xBFWc\xF2\xFD\xE3\x8B\x14a\x01,W_\x80\xFD[4a\x01\xBCW` 6`\x03\x19\x01\x12a\x01\xBCWa\x01Ea\x0F\xADV[a\x01Ma\x17aV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a\x01hWa\x01e\x90a\x19qV[\x80\xF3[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x90\xFD[\x80\xFD[P4a\x01\xBCW` 6`\x03\x19\x01\x12a\x01\xBCW` \x90`\xFF\x90`@\x90`\x01`\x01`\xA0\x1B\x03a\x01\xEAa\x0F\xADV[\x16\x81R`n\x84R T\x16`@Q\x90\x15\x15\x81R\xF3[P4a\x01\xBCW`@6`\x03\x19\x01\x12a\x01\xBCW`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x02lWa\x02/\x906\x90`\x04\x01a\x10\xFEV[`$5\x90`\x01`\x01`@\x1B\x03\x82\x11a\x02hWa\x02ca\x02Ua\x01e\x936\x90`\x04\x01a\x0F\xC3V[\x91a\x02^a\x17aV[a\x1A\x9EV[a\x14\xB4V[\x82\x80\xFD[P\x80\xFD[P4a\x01\xBCW\x80`\x03\x196\x01\x12a\x01\xBCW`lT\x90\x81a\x02\xA2W` \x91P[`@Q`\x01`\x01`\xE0\x1B\x03\x91\x90\x91\x16\x81R\xF3[_\x19\x82\x01\x91\x82\x11a\x02\xC4WPa\x02\xBB` \x91`la\x15\x03V[PT\x81\x1Ca\x02\x8FV[cNH{q`\xE0\x1B\x81R`\x11`\x04R`$\x90\xFD[P4a\x01\xBCW``6`\x03\x19\x01\x12a\x01\xBCWa\x02\xF2a\x0F\xADV[`D5`\x01`\x01`@\x1B\x03\x81\x11a\x02hWa\x03\x11\x906\x90`\x04\x01a\x10\xFEV[\x82T\x90`\xFF\x82`\x08\x1C\x16\x15\x92\x83\x80\x94a\x04`W[\x80\x15a\x04IW[\x15a\x03\xEDW`\xFF\x19\x83\x16`\x01\x17\x85Ua\x03|\x92\x84a\x03\xDCW[Pa\x03V`\xFF\x86T`\x08\x1C\x16a\x1A>V[`\x01\x80`\xA0\x1B\x03\x16`\x01`\x01``\x1B\x03`\xA0\x1B`hT\x16\x17`hUa\x02^`$5a\x17\xB9V[a\x03\x95`\xFF\x83T`\x08\x1C\x16a\x03\x90\x81a\x1A>V[a\x1A>V[a\x03\x9E3a\x19qV[a\x03\xA5W\x80\xF3[a\xFF\0\x19\x81T\x16\x81U\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98` `@Q`\x01\x81R\xA1\x80\xF3[a\xFF\xFF\x19\x16a\x01\x01\x17\x85U_a\x03EV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x90\xFD[P0;\x15\x80\x15a\x03,WP`\x01`\xFF\x84\x16\x14a\x03,V[P`\x01`\xFF\x84\x16\x10a\x03%V[P4a\x01\xBCW` 6`\x03\x19\x01\x12a\x01\xBCW` \x90`\x01`\x01`\xA0\x1B\x03a\x04\x92a\x0F\xADV[\x16\x81R`j\x82R`@\x90 `\x01`\x01`\xA0\x1B\x03\x90a\x04\xAF\x90a\x17/V[\x16`@Q\x90\x81R\xF3[P4a\x01\xBCW` 6`\x03\x19\x01\x12a\x01\xBCW` a\x04\xDCa\x04\xD7a\x0F\xADV[a\x12{V[`@Q\x90\x81R\xF3[P4a\x01\xBCW`@6`\x03\x19\x01\x12a\x01\xBCWa\x04\xFEa\x0F\xADV[`$5\x90c\xFF\xFF\xFF\xFF\x82\x16\x80\x92\x03a\x02hW`\x01`\x01`\xA0\x1B\x03\x16\x82R`m` \x90\x81R`@\x90\x92 a\x04\xDC\x91\x90a\x16\x92V[P4a\x01\xBCW\x80`\x03\x196\x01\x12a\x01\xBCW`3T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[P4a\x01\xBCW\x80`\x03\x196\x01\x12a\x01\xBCW3\x81R`n` R`\xFF`@\x82 T\x16\x15a\x06`W`eT\x80\x15a\x06LW_\x19\x01`eU3\x81R`n` R`@\x81 `\xFF\x19\x81T\x16\x90Ua\x05\xB4a\x05\xAF3a\x1C\x03V[a\x1E\0V[PP`hT\x81\x90`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x06IW\x81\x80\x91`$`@Q\x80\x94\x81\x93cQ\xB2zm`\xE1\x1B\x83R3`\x04\x84\x01RZ\xF1\x80\x15a\x06>Wa\x06)W[P`hT`\x01`\x01`\xA0\x1B\x03\x163\x7F1\xE0\xAD\xFE\xC7\x1B\xCC\xEE7\xB6\xE8:\x90\xC2\xFE\xDB\x17\xD8\xF1i?\xEE\x86<Gq\xE7\xBF\xE2\xAE\xD5\x80\x83\x80\xA3\x80\xF3[\x81a\x063\x91a\x0FuV[a\x01\xBCW\x80_a\x05\xF4V[`@Q=\x84\x82>=\x90\xFD[P\xFD[cNH{q`\xE0\x1B\x82R`\x11`\x04R`$\x82\xFD[c%\xECl\x1F`\xE0\x1B\x81R`\x04\x90\xFD[P4a\x01\xBCW` 6`\x03\x19\x01\x12a\x01\xBCWa\x06\x89a\x0F\xADV[3\x82R`n` R`\xFF`@\x83 T\x16\x15a\x06\xA8Wa\x01e\x903a\x19\xB9V[c%\xECl\x1F`\xE0\x1B\x82R`\x04\x82\xFD[P4a\x01\xBCW\x80`\x03\x196\x01\x12a\x01\xBCWa\x06\xD0a\x17aV[`3\x80T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16\x90\x91U\x81\x90`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x82\x80\xA3\x80\xF3[P4a\x01\xBCW`@6`\x03\x19\x01\x12a\x01\xBCW`$5`\x045`\x01`\x01`@\x1B\x03\x82\x11a\x02hW\x7Fq<\xA5;\x88\xD6\xEBc\xF5\xB1\x85L\xB8\xCB\xDDsn\xC5\x1E\xDA\"^Fy\x1A\xA9)\x8B\x01`d\x8F`@a\x07na\x01e\x946\x90`\x04\x01a\x0F\xC3V[\x92a\x07wa\x17aV[`gT\x90\x80`gU\x82Q\x91\x82R` \x82\x01R\xA1a\x14\xB4V[P4a\x01\xBCW` 6`\x03\x19\x01\x12a\x01\xBCWa\x07\xA9a\x17aV[a\x01e`\x045a\x17\xB9V[P4a\x01\xBCW`@6`\x03\x19\x01\x12a\x01\xBCW` \x90`\x01`\x01`\xA0\x1B\x03a\x07\xD9a\x0F\xADV[\x16\x81R`j\x82R`@\x90 `\x01`\x01`\xA0\x1B\x03\x90a\x04\xAF\x90`$5\x90a\x16\x92V[P4a\x01\xBCW`@6`\x03\x19\x01\x12a\x01\xBCW`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x02lW6`#\x82\x01\x12\x15a\x02lW\x80`\x04\x015\x90a\x087\x82a\x0F\x96V[\x90a\x08E`@Q\x92\x83a\x0FuV[\x82\x82R` \x82\x01\x92`\x05\x1B\x81\x01`$\x01\x836\x82\x11a\t\x02W`$\x83\x01\x90[\x82\x82\x10a\x08\xCFWPPPP`$5`\x01`\x01`@\x1B\x03\x81\x11a\x08\xCBWa\x08\x8D\x906\x90`\x04\x01a\x10_V[PQ\x15a\x08\xB7WQ\x80Q`eT\x03a\x08\xA8Wa\x01e\x90a\x14\xB4V[c\x16\x9E\xFB[`\xE1\x1B\x82R`\x04\x82\xFD[cNH{q`\xE0\x1B\x82R`2`\x04R`$\x82\xFD[\x83\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11a\x08\xFEW` \x91a\x08\xF3\x83\x92`$6\x91\x89\x01\x01a\x0F\xC3V[\x81R\x01\x91\x01\x90a\x08cV[\x87\x80\xFD[\x85\x80\xFD[P4a\x01\xBCW\x80`\x03\x196\x01\x12a\x01\xBCW` `gT`@Q\x90\x81R\xF3[P4a\n\xD2W`@6`\x03\x19\x01\x12a\n\xD2W`\x045`\x01`\x01`@\x1B\x03\x81\x11a\n\xD2W```\x03\x19\x826\x03\x01\x12a\n\xD2W`@Q``\x81\x01\x81\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\n\xF9W`@R\x81`\x04\x015`\x01`\x01`@\x1B\x03\x81\x11a\n\xD2Wa\t\x94\x90`\x046\x91\x85\x01\x01a\x10_V[\x81R`$\x80\x83\x015` \x83\x01\x90\x81R`D\x90\x93\x015`@\x83\x01\x90\x81R\x905`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\n\xD2W3_R`n` R`\xFF`@_ T\x16a\n\xEAW`eT\x90_\x19\x82\x14a\n\xD6W`\x01a\n\x17\x92\x01`eU3_R`n` R`@_ `\x01`\xFF\x19\x82T\x16\x17\x90Ua\n\x0Fa\x05\xAF3a\x1C\x03V[PP3a\x19\xB9V[`hT`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\n\xD2Wa\nc_\x80\x94`@Q\x96\x87\x95\x86\x94\x85\x93c\x99&\xEE}`\xE0\x1B\x85R3`\x04\x86\x01R`@`$\x86\x01RQ```D\x86\x01R`\xA4\x85\x01\x90a\x17=V[\x91Q`d\x84\x01RQ`\x84\x83\x01R\x03\x92Z\xF1\x80\x15a\n\xC7Wa\n\xB4W[P`hT`\x01`\x01`\xA0\x1B\x03\x163\x7F\xA4S\xDBa*\xF5\x9EU!\xD6\xAB\x92\x84\xDC>-\x06\xAF(n\xB1\xB1\xB7\xB7q\xFC\xE4ql\x19\xF2\xC1\x83\x80\xA3\x80\xF3[a\n\xC0\x91P_\x90a\x0FuV[__a\n\x7FV[`@Q=_\x82>=\x90\xFD[_\x80\xFD[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[cB\xEEh\xB5`\xE0\x1B_R`\x04_\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[4a\n\xD2W` 6`\x03\x19\x01\x12a\n\xD2W`\x01`\x01`\xA0\x1B\x03a\x0B.a\x0F\xADV[\x16_R`m` R` a\x04\xDC`@_ a\x17/V[4a\n\xD2W_6`\x03\x19\x01\x12a\n\xD2W` a\x04\xDCa\x17\"V[4a\n\xD2W` 6`\x03\x19\x01\x12a\n\xD2W` a\x04\xDCc\xFF\xFF\xFF\xFFa\x0B\x81a\x101V[\x16a\x16\x01V[4a\n\xD2W_6`\x03\x19\x01\x12a\n\xD2W```@Qa\x0B\xA5\x81a\x0F?V[Ra\x0B\xD3`@Qa\x0B\xB5\x81a\x0F?V[a\x0B\xBDa\x11\xCFV[\x81R`@Q\x91\x82\x91` \x83R` \x83\x01\x90a\x10\xA5V[\x03\x90\xF3[4a\n\xD2W`@6`\x03\x19\x01\x12a\n\xD2W`\x045`$5`\x01`\x01`@\x1B\x03\x81\x11a\n\xD2Wa\x0C\n\x906\x90`\x04\x01a\x10_V[\x80Q\x81\x01``\x82` \x83\x01\x92\x03\x12a\n\xD2W` \x82\x01Q`\x01`\x01`@\x1B\x03\x81\x11a\n\xD2W\x82\x01\x91\x81`?\x84\x01\x12\x15a\n\xD2W` \x83\x01Qa\x0CK\x81a\x0F\x96V[\x93a\x0CY`@Q\x95\x86a\x0FuV[\x81\x85R` \x80\x80\x87\x01\x93`\x05\x1B\x83\x01\x01\x01\x90\x84\x82\x11a\n\xD2W`@\x01\x91[\x81\x83\x10a\x0E\xC1WPPP`@\x81\x01Q`\x01`\x01`@\x1B\x03\x81\x11a\n\xD2W\x81\x01\x91\x80`?\x84\x01\x12\x15a\n\xD2W` \x83\x01Q\x92a\x0C\xB1\x84a\x0F\x96V[\x93a\x0C\xBF`@Q\x95\x86a\x0FuV[\x80\x85R` \x80\x80\x87\x01\x92`\x05\x1B\x84\x01\x01\x01\x91\x83\x83\x11a\n\xD2W`@\x81\x01\x91[\x83\x83\x10a\x0EWW\x88\x88\x88``\x89\x01Q\x91c\xFF\xFF\xFF\xFF\x83\x16\x80\x93\x03a\n\xD2W\x80Q\x91__\x95\x82Q\x85\x03a\x0EHW\x84\x15a\x0E9WC\x86\x10\x15\x94_\x94\x90\x93\x92[\x84\x86\x10a\r\x85W\x88\x88\x88a\rvWa\r2\x81a\x15\x18V[\x82\x11a\rgWa\rA\x90a\x16\x01V[\x11a\rXW`@Qc\x0B\x13]?`\xE1\x1B\x81R` \x90\xF3[c\xE1!c/`\xE0\x1B_R`\x04_\xFD[cK\x05\xA0\xF7`\xE1\x1B_R`\x04_\xFD[c\xE6O\x18\x0F`\xE0\x1B_R`\x04_\xFD[\x94\x97\x93\x94\x92\x93\x92`\x01`\x01`\xA0\x1B\x03a\r\x9E\x8A\x84a\x12FV[Q\x16\x96a\rvW\x86_R`j` R\x86`\x01\x80`\xA0\x1B\x03a\r\xC2\x8A`@_ a\x16\x92V[\x16\x91`\x01`\x01`\xA0\x1B\x03\x16\x10\x15a\x0E*Wa\r\xE9\x90\x83a\r\xE2\x8B\x88a\x12FV[Q\x91a \x91V[\x15a\x0E\x1BW`\x01a\x0E\x11\x87\x94_\x98_R`m` Ra\x0E\x0B\x8A`@_ a\x16\x92V[\x90a\x12nV[\x98\x01\x94\x93\x92a\r\x1BV[c\x8B\xAAW\x9F`\xE0\x1B_R`\x04_\xFD[c\xBAP\xF9\x11`\xE0\x1B_R`\x04_\xFD[c%\x1FV\xA1`\xE2\x1B_R`\x04_\xFD[c\x1F\xECgG`\xE3\x1B_R`\x04_\xFD[\x82Q`\x01`\x01`@\x1B\x03\x81\x11a\n\xD2W` \x90\x83\x01\x01\x85`?\x82\x01\x12\x15a\n\xD2W` \x81\x01Q\x91a\x0E\x87\x83a\x10DV[a\x0E\x94`@Q\x91\x82a\x0FuV[\x83\x81R`@\x83\x85\x01\x01\x88\x10a\n\xD2W_` \x85\x81\x96`@\x83\x97\x01\x83\x86\x01^\x83\x01\x01R\x81R\x01\x92\x01\x91a\x0C\xDEV[\x82Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\n\xD2W\x81R` \x92\x83\x01\x92\x01a\x0CwV[4a\n\xD2W` 6`\x03\x19\x01\x12a\n\xD2W` a\x04\xDCc\xFF\xFF\xFF\xFFa\x0F\x04a\x101V[\x16a\x15\x18V[4a\n\xD2W` 6`\x03\x19\x01\x12a\n\xD2W`\x045`\x01`\x01`@\x1B\x03\x81\x11a\n\xD2Wa\x02ca\x0F=\x916\x90`\x04\x01a\x0F\xC3V[\0[` \x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\n\xF9W`@RV[`@\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\n\xF9W`@RV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\n\xF9W`@RV[`\x01`\x01`@\x1B\x03\x81\x11a\n\xF9W`\x05\x1B` \x01\x90V[`\x045\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\n\xD2WV[\x90\x80`\x1F\x83\x01\x12\x15a\n\xD2W\x815\x90a\x0F\xDB\x82a\x0F\x96V[\x92a\x0F\xE9`@Q\x94\x85a\x0FuV[\x82\x84R` \x80\x85\x01\x93`\x05\x1B\x82\x01\x01\x91\x82\x11a\n\xD2W` \x01\x91[\x81\x83\x10a\x10\x11WPPP\x90V[\x825`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\n\xD2W\x81R` \x92\x83\x01\x92\x01a\x10\x04V[`\x045\x90c\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\n\xD2WV[`\x01`\x01`@\x1B\x03\x81\x11a\n\xF9W`\x1F\x01`\x1F\x19\x16` \x01\x90V[\x81`\x1F\x82\x01\x12\x15a\n\xD2W\x805\x90a\x10v\x82a\x10DV[\x92a\x10\x84`@Q\x94\x85a\x0FuV[\x82\x84R` \x83\x83\x01\x01\x11a\n\xD2W\x81_\x92` \x80\x93\x01\x83\x86\x017\x83\x01\x01R\x90V[` `@\x81\x84\x01\x92Q\x93\x82\x81R\x84Q\x80\x94R\x01\x92\x01\x90_[\x81\x81\x10a\x10\xCAWPPP\x90V[\x82Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x85R` \x90\x81\x01Q`\x01`\x01``\x1B\x03\x16\x81\x86\x01R`@\x90\x94\x01\x93\x90\x92\x01\x91`\x01\x01a\x10\xBDV[\x91\x90` \x83\x82\x03\x12a\n\xD2W`@Qa\x11\x16\x81a\x0F?V[\x80\x93\x805\x90`\x01`\x01`@\x1B\x03\x82\x11a\n\xD2W\x01\x82`\x1F\x82\x01\x12\x15a\n\xD2W\x805\x90a\x11A\x82a\x0F\x96V[\x93a\x11O`@Q\x95\x86a\x0FuV[\x82\x85R` \x80\x86\x01\x93`\x06\x1B\x83\x01\x01\x91\x81\x83\x11a\n\xD2W` \x01\x92[\x82\x84\x10a\x11yWPPPPRV[`@\x84\x83\x03\x12a\n\xD2W`@Q\x90a\x11\x90\x82a\x0FZV[\x845`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\n\xD2W\x82R` \x85\x015\x90`\x01`\x01``\x1B\x03\x82\x16\x82\x03a\n\xD2W\x82` \x92\x83`@\x95\x01R\x81R\x01\x93\x01\x92a\x11kV[`fT\x90a\x11\xDC\x82a\x0F\x96V[\x91a\x11\xEA`@Q\x93\x84a\x0FuV[\x80\x83R`f_\x90\x81R_Q` a#)_9_Q\x90_R` \x85\x01[\x83\x83\x10a\x12\x13WPPPPV[`\x01` \x81\x92`@Qa\x12%\x81a\x0FZV[\x85T\x84\x80`\xA0\x1B\x03\x81\x16\x82R`\xA0\x1C\x83\x82\x01R\x81R\x01\x92\x01\x92\x01\x91\x90a\x12\x06V[\x80Q\x82\x10\x15a\x12ZW` \x91`\x05\x1B\x01\x01\x90V[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[\x91\x90\x82\x01\x80\x92\x11a\n\xD6WV[a\x12\x83a\x11\xCFV[\x80Q\x90\x92_a\x12\xAAa\x12\x94\x84a\x0F\x96V[\x93a\x12\xA2`@Q\x95\x86a\x0FuV[\x80\x85Ra\x0F\x96V[` \x84\x01\x90`\x1F\x19\x016\x827_[\x86Q\x81\x10\x15a\x12\xEAW`\x01\x90`\x01`\x01`\xA0\x1B\x03a\x12\xD6\x82\x8Aa\x12FV[QQ\x16a\x12\xE3\x82\x88a\x12FV[R\x01a\x12\xB8V[P`@\x80Qc\x90\x04\x13G`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x96\x16`\x04\x87\x01R`$\x86\x01R\x92Q`D\x85\x01\x81\x90R\x91\x93\x90\x92\x90\x91\x82\x91`d\x83\x01\x91_[\x81\x81\x10a\x14wWP_\x93\x92\x83\x90\x03\x91P\x82\x90P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\n\xC7W_\x91a\x13\xE7W[P_\x91[\x84Q\x83\x10\x15a\x13\xC9Wa\x13\x8A\x83\x83a\x12FV[Q\x90`\x01`\x01``\x1B\x03` a\x13\xA0\x86\x89a\x12FV[Q\x01Q\x16\x91\x82\x81\x02\x92\x81\x84\x04\x14\x90\x15\x17\x15a\n\xD6W`\x01\x91a\x13\xC1\x91a\x12nV[\x92\x01\x91a\x13wV[\x91PPa'\x10\x91\x92P\x04`gT\x81\x10\x15_\x14a\x13\xE2W\x90V[P_\x90V[\x90P=\x80_\x83>a\x13\xF8\x81\x83a\x0FuV[\x81\x01\x90` \x81\x83\x03\x12a\n\xD2W\x80Q\x90`\x01`\x01`@\x1B\x03\x82\x11a\n\xD2W\x01\x90\x80`\x1F\x83\x01\x12\x15a\n\xD2W\x81Qa\x14.\x81a\x0F\x96V[\x92a\x14<`@Q\x94\x85a\x0FuV[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\n\xD2W` \x01\x90[\x82\x82\x10a\x14gWPPP_a\x13sV[\x81Q\x81R` \x91\x82\x01\x91\x01a\x14WV[\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x84R\x85\x94P` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x13&V[\x91\x90\x91_\x83\x82\x01\x93\x84\x12\x91\x12\x90\x80\x15\x82\x16\x91\x15\x16\x17a\n\xD6WV[\x90_\x80[\x83Q\x82\x10\x15a\x14\xF2W`\x01\x90a\x14\xEA\x90a\x14\xE4`\x01`\x01`\xA0\x1B\x03a\x14\xDD\x86\x89a\x12FV[Q\x16a\x1C\x03V[\x90a\x14\x99V[\x91\x01\x90a\x14\xB8V[\x90Pa\x14\xFF\x91\x92Pa\x1E\0V[PPV[\x80T\x82\x10\x15a\x12ZW_R` _ \x01\x90_\x90V[C\x81\x10\x15a\x15\xBDW`kT\x90_\x90[\x82\x82\x10a\x15dWPP\x80a\x15DWP_[`\x01`\x01`\xE0\x1B\x03\x16\x90V[_\x19\x81\x01\x90\x81\x11a\n\xD6Wa\x15Z\x90`ka\x15\x03V[PT` \x1Ca\x158V[\x90\x91a\x15w\x81\x84\x18`\x01\x1C\x82\x85\x16a\x12nV[\x90\x82c\xFF\xFF\xFF\xFFa\x15\x89\x84`ka\x15\x03V[PT\x16\x11\x15a\x15\x9BWP\x91[\x90a\x15'V[\x92P`\x01\x81\x01\x80\x91\x11\x15a\x15\x95WcNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[`d`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R` `$\x82\x01R\x7FCheckpoints: block not yet mined`D\x82\x01R\xFD[C\x81\x10\x15a\x15\xBDW`lT\x90_\x90[\x82\x82\x10a\x169WPP\x80a\x16#WP_\x90V[_\x19\x81\x01\x90\x81\x11a\n\xD6Wa\x15Z\x90`la\x15\x03V[\x90\x91a\x16L\x81\x84\x18`\x01\x1C\x82\x85\x16a\x12nV[\x90\x82c\xFF\xFF\xFF\xFFa\x16^\x84`la\x15\x03V[PT\x16\x11\x15a\x16pWP\x91[\x90a\x16\x10V[\x92P`\x01\x81\x01\x80\x91\x11\x15a\x16jWcNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x90C\x81\x10\x15a\x15\xBDW\x81T\x90_\x90[\x82\x82\x10a\x16\xCAWPP\x80a\x16\xB6WP_\x91\x90PV[_\x19\x81\x01\x90\x81\x11a\n\xD6Wa\x15Z\x91a\x15\x03V[\x90\x91a\x16\xDD\x81\x84\x18`\x01\x1C\x82\x85\x16a\x12nV[\x90\x82c\xFF\xFF\xFF\xFFa\x16\xEE\x84\x88a\x15\x03V[PT\x16\x11\x15a\x17\0WP\x91[\x90a\x16\xA1V[\x92P`\x01\x81\x01\x80\x91\x11\x15a\x16\xFAWcNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[`kT\x80a\x15DWP_\x90V[\x80T\x80a\x16\xB6WP_\x91\x90PV[\x80Q\x80\x83R` \x92\x91\x81\x90\x84\x01\x84\x84\x01^_\x82\x82\x01\x84\x01R`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[`3T`\x01`\x01`\xA0\x1B\x03\x163\x03a\x17uWV[`d`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R` `$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R\xFD[`lT\x90a\x17\xC7`la\x17/V[\x90\x82\x15\x15\x80a\x19KW[\x15a\x18?Wa\x17\xDF\x81a!\xA9V[\x91_\x19\x84\x01\x93\x84\x11a\n\xD6Wa\x185` \x93a\x18\x1D\x7F\x93$\xF7\xE5\xA7\xC0(\x88\x08\xA64\xCC\xDED\xB8\xE9ygdt\xB2.)\xEE\x9D\xD5i\xB5^y\x1AK\x96`la\x15\x03V[P\x90c\xFF\xFF\xFF\xFF\x82T\x91\x81\x19\x90` \x1B\x16\x91\x16\x17\x90UV[P`@Q\x90\x81R\xA1V[c\xFF\xFF\xFF\xFFC\x11a\x18\xF7Wa\x18S\x81a!\xA9V[\x91`@Q\x93a\x18a\x85a\x0FZV[Cc\xFF\xFF\xFF\xFF\x16\x85R`\x01`\x01`\xE0\x1B\x03\x90\x93\x16` \x85\x01\x90\x81R\x92`\x01`@\x1B\x81\x10\x15a\n\xF9W\x80`\x01a\x18\x9B\x92\x01`lU`la\x15\x03V[\x94\x90\x94a\x18\xE4WQ\x92Qc\xFF\xFF\xFF\xFF\x90\x93\x16c\xFF\xFF\xFF\xFF\x19` \x94\x85\x1B\x16\x17\x90\x93U\x7F\x93$\xF7\xE5\xA7\xC0(\x88\x08\xA64\xCC\xDED\xB8\xE9ygdt\xB2.)\xEE\x9D\xD5i\xB5^y\x1AK\x92a\x185V[cNH{q`\xE0\x1B_R_`\x04R`$_\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FSafeCast: value doesn't fit in 3`D\x82\x01Re2 bits`\xD0\x1B`d\x82\x01R`\x84\x90\xFD[P_\x19\x83\x01\x83\x81\x11a\n\xD6Wa\x19gc\xFF\xFF\xFF\xFF\x91`la\x15\x03V[PT\x16C\x14a\x17\xD1V[`3\x80T`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x01`\x01`\xA0\x1B\x03\x19\x82\x16\x81\x17\x90\x92U\x90\x91\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0_\x80\xA3V[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16_\x81\x81R`j` R`@\x90 \x90\x92\x91\x90a\x19\xDE\x90a\x17/V[`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x92\x91\x16\x90\x82\x82\x14a\x1A9W\x80_R`j` Ra\x1A\t\x83`@_ a\x1F.V[PP`@Q\x91\x82R\x7F\xD0a\x16\x82R\xF4As6X\xF0\x9EM\x8F[-\x99\x8E\xD4\xEF$\xA2\xBB\xFDl\xEC\xA5.\xA11P\x02` C\x93\xA4V[PPPV[\x15a\x1AEWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x90\xFD[a\x1A\xA7\x81a \x17V[\x15a\x1B\xDCW`@Q\x91a\x1A\xB9\x83a\x0F?V[a\x1A\xC1a\x11\xCFV[\x83R`fT_`fU\x80a\x1B\xA2W[P_[\x82Q\x80Q\x82\x10\x15a\x1BOW\x81a\x1A\xE8\x91a\x12FV[Q\x90`fT\x91`\x01`@\x1B\x83\x10\x15a\n\xF9W`\x01\x83\x01`fU`fT\x83\x10\x15a\x12ZW`f_R\x80Q` \x91\x90\x91\x01Q`\xA0\x1B`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x17_Q` a#)_9_Q\x90_R\x90\x92\x01\x91\x90\x91U`\x01\x01a\x1A\xD3V[PP\x91a\x1B\x8F\x90a\x1B\x9D\x7F#\xAA\xD4\xE6\x17D\xEC\xE1d\x13\n\xA4\x15\xC1an\x80\x13k\x0F\x07p\xE5e\x89C\x8B\x90\xB2i&^\x93`@Q\x93\x84\x93`@\x85R`@\x85\x01\x90a\x10\xA5V[\x90\x83\x82\x03` \x85\x01Ra\x10\xA5V[\x03\x90\xA1V[`f_R_Q` a#)_9_Q\x90_R\x01_Q` a#)_9_Q\x90_R[\x81\x81\x10a\x1B\xD1WPa\x1A\xD0V[_\x81U`\x01\x01a\x1B\xC4V[c\xD1sWy`\xE0\x1B_R`\x04_\xFD[\x81\x81\x03\x92\x91_\x13\x80\x15\x82\x85\x13\x16\x91\x84\x12\x16\x17a\n\xD6WV[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x81\x81R`m` R`@\x81 \x90\x92\x90\x83\x90a\x1C(\x90a\x17/V[\x80\x94\x84_R`n` R`\xFF`@_ T\x16\x15_\x14a\x1D\xA2Wa\x1CL\x92\x93Pa\x1B\xEBV[\x92\x83\x15a\x1D\x9CW\x82_R`m` R`@_ \x90\x81T\x92a\x1Cl\x83a\x17/V[\x84\x15\x15\x80a\x1DwW[\x15a\x1C\xD0Wa\x1C\x83_a!\xA9V[\x94_\x19\x81\x01\x90\x81\x11a\n\xD6W\x7F\x88w\r\xC8b\xE4z~\xD5\x86\x90xW\xEB\x1Bu\xE4\xC5\xFF\xC8\xB7\x07\xC7\xEE\x10\xEBt\xD6\x88_\xE5\x94\x95a\x18\x1Da\x1C\xC0\x92`@\x97a\x15\x03V[P[\x82Q\x91\x82R` \x82\x01R\xA2\x90V[c\xFF\xFF\xFF\xFFC\x11a\x18\xF7Wa\x1C\xE4_a!\xA9V[\x93`@Q\x95a\x1C\xF2\x87a\x0FZV[Cc\xFF\xFF\xFF\xFF\x16\x87R`\x01`\x01`\xE0\x1B\x03\x90\x95\x16` \x87\x01\x90\x81R\x94`\x01`@\x1B\x81\x10\x15a\n\xF9Wa\x1D)\x91`\x01\x82\x01\x81Ua\x15\x03V[\x95\x90\x95a\x18\xE4WQ\x93Q` \x1Bc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x94\x90\x94\x16\x93\x90\x93\x17\x90\x93U\x7F\x88w\r\xC8b\xE4z~\xD5\x86\x90xW\xEB\x1Bu\xE4\xC5\xFF\xC8\xB7\x07\xC7\xEE\x10\xEBt\xD6\x88_\xE5\x94\x92`@\x92a\x1C\xC0V[P_\x19\x85\x01\x85\x81\x11a\n\xD6Wa\x1D\x92c\xFF\xFF\xFF\xFF\x91\x86a\x15\x03V[PT\x16C\x14a\x1CuV[PPP\x90V[PPPa\x1D\xAE\x90a\x12{V[\x91a\x1D\xB9\x81\x84a\x1B\xEBV[\x92\x83\x15a\x1D\x9CW\x7F\x88w\r\xC8b\xE4z~\xD5\x86\x90xW\xEB\x1Bu\xE4\xC5\xFF\xC8\xB7\x07\xC7\xEE\x10\xEBt\xD6\x88_\xE5\x94\x91`@\x91\x84_R`m` Ra\x1D\xF9\x81\x84_ a\x1F.V[PPa\x1C\xC2V[\x90a\x1E\x13a\x1E\x0Ca\x17\"V[\x92\x83a\x14\x99V[\x90`kTa\x1E!`ka\x17/V[\x90\x80\x15\x15\x80a\x1F\x08W[\x15a\x1E\x84Wa\x1E9\x84a!\xA9V[_\x19\x82\x01\x91\x82\x11a\n\xD6Wa\x18\x1Da\x1ER\x92`ka\x15\x03V[P\x7F\x86\xDC\xF8k\x12\xDF\xEE\xDE\xA7J\xE90\r\xBD\xAA\x19;\xCC\xE5\x80\x93i\xC8\x17~\xA2\xF4\xEA\xAAer\x9B`@\x80Q\x85\x81R\x84` \x82\x01R\xA1V[c\xFF\xFF\xFF\xFFC\x11a\x18\xF7Wa\x1E\x98\x84a!\xA9V[`@Q\x91a\x1E\xA5\x83a\x0FZV[Cc\xFF\xFF\xFF\xFF\x16\x83R`\x01`\x01`\xE0\x1B\x03\x90\x91\x16` \x83\x01\x90\x81R\x90`\x01`@\x1B\x81\x10\x15a\n\xF9W\x80`\x01a\x1E\xDF\x92\x01`kU`ka\x15\x03V[\x91\x90\x91a\x18\xE4W\x91Q\x91Q` \x1Bc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90Ua\x1ERV[P_\x19\x81\x01\x81\x81\x11a\n\xD6Wa\x1F$c\xFF\xFF\xFF\xFF\x91`ka\x15\x03V[PT\x16C\x14a\x1E+V[\x91\x90\x91\x80Ta\x1F<\x82a\x17/V[\x91\x81\x15\x15\x80a\x1F\xF2W[\x15a\x1FqWa\x1FT\x85a!\xA9V[\x90_\x19\x83\x01\x92\x83\x11a\n\xD6Wa\x1Fm\x92a\x18\x1D\x91a\x15\x03V[\x91\x90V[c\xFF\xFF\xFF\xFFC\x11a\x18\xF7Wa\x1F\x85\x85a!\xA9V[\x90`@Q\x92a\x1F\x93\x84a\x0FZV[Cc\xFF\xFF\xFF\xFF\x16\x84R`\x01`\x01`\xE0\x1B\x03\x90\x92\x16` \x84\x01\x90\x81R\x91`\x01`@\x1B\x81\x10\x15a\n\xF9Wa\x1F\xCA\x91`\x01\x82\x01\x81Ua\x15\x03V[\x91\x90\x91a\x18\xE4W\x91Q\x91Q` \x1Bc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x91\x90V[P_\x19\x82\x01\x82\x81\x11a\n\xD6Wa \rc\xFF\xFF\xFF\xFF\x91\x83a\x15\x03V[PT\x16C\x14a\x1FFV[Q\x90_\x80\x80[\x84Q\x82\x10\x15a zW`\x01`\x01`\xA0\x1B\x03a 8\x83\x87a\x12FV[QQ\x16\x90`\x01`\x01`\xA0\x1B\x03\x16\x81\x11\x15a\x0E*Wa q`\x01\x91\x93`\x01`\x01``\x1B\x03` a g\x86\x8Aa\x12FV[Q\x01Q\x16\x90a\x12nV[\x91\x01\x90\x91a \x1DV[P\x91\x92Pa'\x10\x14\x90Pa \x8CW_\x90V[`\x01\x90V[\x91\x90\x91a \x9E\x82\x84a\"\x18V[`\x05\x81\x10\x15a!\x95W\x15\x90\x81a!\x7FW[Pa!wW_\x92a \xE9a \xF7\x85\x94`@Q\x92\x83\x91` \x83\x01\x95c\x0B\x13]?`\xE1\x1B\x87R`$\x84\x01R`@`D\x84\x01R`d\x83\x01\x90a\x17=V[\x03`\x1F\x19\x81\x01\x83R\x82a\x0FuV[Q\x91Z\xFA=\x15a!pW=a!\x0B\x81a\x10DV[\x90a!\x19`@Q\x92\x83a\x0FuV[\x81R=_` \x83\x01>[\x81a!dW[\x81a!2WP\x90V[\x90P` \x81\x80Q\x81\x01\x03\x12a\n\xD2W` \x01Q`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x90\x81\x90\x03a\n\xD2Wc\x0B\x13]?`\xE1\x1B\x14\x90V[\x80Q` \x14\x91Pa!)V[``a!#V[PPP`\x01\x90V[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16\x91\x16\x14\x90P_a \xAFV[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[`\x01`\x01`\xE0\x1B\x03\x81\x11a!\xC3W`\x01`\x01`\xE0\x1B\x03\x16\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FSafeCast: value doesn't fit in 2`D\x82\x01Rf24 bits`\xC8\x1B`d\x82\x01R`\x84\x90\xFD[\x81Q`A\x81\x03a\"DWP\x90a\"@\x91` \x82\x01Q\x90```@\x84\x01Q\x93\x01Q_\x1A\x90a\"\x86V[\x90\x91V[`@\x03a\"}W`@` \x83\x01Q\x92\x01Q\x91\x82`\xFF\x1C\x91`\x1B\x83\x01\x80\x93\x11a\n\xD6Wa\"@\x93`\x01`\x01`\xFF\x1B\x03\x16\x92`\xFF\x16\x90a\"\x86V[PP_\x90`\x02\x90V[\x90\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x84\x11a#\x1DW`\xFF\x16\x90`\x1B\x82\x14\x15\x80a#\x12W[a#\x07W` \x93_\x93`\x80\x93`@Q\x93\x84R\x86\x84\x01R`@\x83\x01R``\x82\x01R\x82\x80R`\x01Z\xFA\x15a\n\xC7W_Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a\"\xFFW\x90_\x90V[P_\x90`\x01\x90V[PPPP_\x90`\x04\x90V[P`\x1C\x82\x14\x15a\"\xBDV[PPPP_\x90`\x03\x90V\xFEFP\x18y\xB8\xCA\x85%\xE8\xC2\xFDQ\x9E/\xBF\xCF\xA2\xEB\xEA&P\x12\x94\xAA\x02\xCB\xFC\xFB\x12\xE9CT\xA2dipfsX\"\x12 \xD4}\xE4>n\x0E\x95\x1AtS \x99\x04\x0E\x81o\x814\xAFR\x94\xBF\xF6\x96[E;\x99DQv)dsolcC\0\x08\x1B\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x60806040526004361015610011575f80fd5b5f5f3560e01c8062cf2ab514610f0a5780630dba339414610ee15780631626ba7e14610bd75780631703a01814610b875780631e4cd85e14610b5e578063314f3a4914610b445780633b242e4a14610b0d5780633d5611f61461092457806340bf2fb7146109065780635140a548146107fa5780635e1042e8146107b45780635ef533291461078f578063696255be14610714578063715018a6146106b7578063743c31f41461066f578063857dc1901461055a5780638da5cb5b14610531578063955f2d90146104e457806398ec1ac9146104b8578063a2ce5fd11461046d578063ab118995146102d8578063b933fa7414610270578063dec5d1f6146101fe578063ec7fbb31146101bf5763f2fde38b1461012c575f80fd5b346101bc5760203660031901126101bc57610145610fad565b61014d611761565b6001600160a01b038116156101685761016590611971565b80f35b60405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b6064820152608490fd5b80fd5b50346101bc5760203660031901126101bc5760209060ff906040906001600160a01b036101ea610fad565b168152606e84522054166040519015158152f35b50346101bc5760403660031901126101bc576004356001600160401b03811161026c5761022f9036906004016110fe565b602435906001600160401b03821161026857610263610255610165933690600401610fc3565b9161025e611761565b611a9e565b6114b4565b8280fd5b5080fd5b50346101bc57806003193601126101bc57606c5490816102a257602091505b6040516001600160e01b03919091168152f35b5f1982019182116102c457506102bb602091606c611503565b5054811c61028f565b634e487b7160e01b81526011600452602490fd5b50346101bc5760603660031901126101bc576102f2610fad565b6044356001600160401b038111610268576103119036906004016110fe565b82549060ff8260081c161592838094610460575b8015610449575b156103ed5760ff198316600117855561037c92846103dc575b5061035660ff865460081c16611a3e565b60018060a01b03166001600160601b0360a01b606854161760685561025e6024356117b9565b61039560ff835460081c1661039081611a3e565b611a3e565b61039e33611971565b6103a55780f35b61ff001981541681557f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb3847402498602060405160018152a180f35b61ffff19166101011785555f610345565b60405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b6064820152608490fd5b50303b15801561032c5750600160ff84161461032c565b50600160ff841610610325565b50346101bc5760203660031901126101bc576020906001600160a01b03610492610fad565b168152606a8252604090206001600160a01b03906104af9061172f565b16604051908152f35b50346101bc5760203660031901126101bc5760206104dc6104d7610fad565b61127b565b604051908152f35b50346101bc5760403660031901126101bc576104fe610fad565b6024359063ffffffff8216809203610268576001600160a01b03168252606d602090815260409092206104dc9190611692565b50346101bc57806003193601126101bc576033546040516001600160a01b039091168152602090f35b50346101bc57806003193601126101bc57338152606e60205260ff6040822054161561066057606554801561064c575f1901606555338152606e6020526040812060ff1981541690556105b46105af33611c03565b611e00565b505060685481906001600160a01b0316803b15610649578180916024604051809481936351b27a6d60e11b83523360048401525af1801561063e57610629575b506068546001600160a01b0316337f31e0adfec71bccee37b6e83a90c2fedb17d8f1693fee863c4771e7bfe2aed5808380a380f35b8161063391610f75565b6101bc57805f6105f4565b6040513d84823e3d90fd5b50fd5b634e487b7160e01b82526011600452602482fd5b6325ec6c1f60e01b8152600490fd5b50346101bc5760203660031901126101bc57610689610fad565b338252606e60205260ff604083205416156106a85761016590336119b9565b6325ec6c1f60e01b8252600482fd5b50346101bc57806003193601126101bc576106d0611761565b603380546001600160a01b0319811690915581906001600160a01b03167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e08280a380f35b50346101bc5760403660031901126101bc576024356004356001600160401b038211610268577f713ca53b88d6eb63f5b1854cb8cbdd736ec51eda225e46791aa9298b0160648f604061076e610165943690600401610fc3565b92610777611761565b606754908060675582519182526020820152a16114b4565b50346101bc5760203660031901126101bc576107a9611761565b6101656004356117b9565b50346101bc5760403660031901126101bc576020906001600160a01b036107d9610fad565b168152606a8252604090206001600160a01b03906104af9060243590611692565b50346101bc5760403660031901126101bc576004356001600160401b03811161026c573660238201121561026c5780600401359061083782610f96565b906108456040519283610f75565b828252602082019260051b8101602401833682116109025760248301905b8282106108cf57505050506024356001600160401b0381116108cb5761088d90369060040161105f565b5051156108b757518051606554036108a857610165906114b4565b63169efb5b60e11b8252600482fd5b634e487b7160e01b82526032600452602482fd5b8380fd5b81356001600160401b0381116108fe576020916108f3839260243691890101610fc3565b815201910190610863565b8780fd5b8580fd5b50346101bc57806003193601126101bc576020606754604051908152f35b5034610ad2576040366003190112610ad2576004356001600160401b038111610ad25760606003198236030112610ad257604051606081018181106001600160401b03821117610af95760405281600401356001600160401b038111610ad257610994906004369185010161105f565b8152602480830135602083019081526044909301356040830190815290356001600160a01b0381168103610ad257335f52606e60205260ff60405f205416610aea57606554905f198214610ad6576001610a179201606555335f52606e60205260405f20600160ff19825416179055610a0f6105af33611c03565b5050336119b9565b6068546001600160a01b0316803b15610ad257610a635f809460405196879586948593639926ee7d60e01b855233600486015260406024860152516060604486015260a485019061173d565b9151606484015251608483015203925af18015610ac757610ab4575b506068546001600160a01b0316337fa453db612af59e5521d6ab9284dc3e2d06af286eb1b1b7b771fce4716c19f2c18380a380f35b610ac091505f90610f75565b5f5f610a7f565b6040513d5f823e3d90fd5b5f80fd5b634e487b7160e01b5f52601160045260245ffd5b6342ee68b560e01b5f5260045ffd5b634e487b7160e01b5f52604160045260245ffd5b34610ad2576020366003190112610ad2576001600160a01b03610b2e610fad565b165f52606d60205260206104dc60405f2061172f565b34610ad2575f366003190112610ad25760206104dc611722565b34610ad2576020366003190112610ad25760206104dc63ffffffff610b81611031565b16611601565b34610ad2575f366003190112610ad2576060604051610ba581610f3f565b52610bd3604051610bb581610f3f565b610bbd6111cf565b81526040519182916020835260208301906110a5565b0390f35b34610ad2576040366003190112610ad2576004356024356001600160401b038111610ad257610c0a90369060040161105f565b8051810160608260208301920312610ad25760208201516001600160401b038111610ad25782019181603f84011215610ad2576020830151610c4b81610f96565b93610c596040519586610f75565b8185526020808087019360051b8301010190848211610ad257604001915b818310610ec15750505060408101516001600160401b038111610ad25781019180603f84011215610ad257602083015192610cb184610f96565b93610cbf6040519586610f75565b8085526020808087019260051b8401010191838311610ad25760408101915b838310610e575788888860608901519163ffffffff8316809303610ad2578051915f5f9582518503610e48578415610e395743861015945f949093925b848610610d8557888888610d7657610d3281611518565b8211610d6757610d4190611601565b11610d5857604051630b135d3f60e11b8152602090f35b63e121632f60e01b5f5260045ffd5b634b05a0f760e11b5f5260045ffd5b63e64f180f60e01b5f5260045ffd5b949793949293926001600160a01b03610d9e8a84611246565b511696610d7657865f52606a6020528660018060a01b03610dc28a60405f20611692565b16916001600160a01b03161015610e2a57610de99083610de28b88611246565b5191612091565b15610e1b576001610e1187945f985f52606d602052610e0b8a60405f20611692565b9061126e565b9801949392610d1b565b638baa579f60e01b5f5260045ffd5b63ba50f91160e01b5f5260045ffd5b63251f56a160e21b5f5260045ffd5b631fec674760e31b5f5260045ffd5b82516001600160401b038111610ad25760209083010185603f82011215610ad257602081015191610e8783611044565b610e946040519182610f75565b8381526040838501018810610ad2575f602085819660408397018386015e83010152815201920191610cde565b82516001600160a01b0381168103610ad257815260209283019201610c77565b34610ad2576020366003190112610ad25760206104dc63ffffffff610f04611031565b16611518565b34610ad2576020366003190112610ad2576004356001600160401b038111610ad257610263610f3d913690600401610fc3565b005b602081019081106001600160401b03821117610af957604052565b604081019081106001600160401b03821117610af957604052565b90601f801991011681019081106001600160401b03821117610af957604052565b6001600160401b038111610af95760051b60200190565b600435906001600160a01b0382168203610ad257565b9080601f83011215610ad257813590610fdb82610f96565b92610fe96040519485610f75565b82845260208085019360051b820101918211610ad257602001915b8183106110115750505090565b82356001600160a01b0381168103610ad257815260209283019201611004565b6004359063ffffffff82168203610ad257565b6001600160401b038111610af957601f01601f191660200190565b81601f82011215610ad25780359061107682611044565b926110846040519485610f75565b82845260208383010111610ad257815f926020809301838601378301015290565b602060408184019251938281528451809452019201905f5b8181106110ca5750505090565b825180516001600160a01b031685526020908101516001600160601b031681860152604090940193909201916001016110bd565b9190602083820312610ad25760405161111681610f3f565b80938035906001600160401b038211610ad2570182601f82011215610ad25780359061114182610f96565b9361114f6040519586610f75565b82855260208086019360061b83010191818311610ad257602001925b828410611179575050505052565b604084830312610ad2576040519061119082610f5a565b84356001600160a01b0381168103610ad25782526020850135906001600160601b0382168203610ad2578260209283604095015281520193019261116b565b606654906111dc82610f96565b916111ea6040519384610f75565b80835260665f9081525f5160206123295f395f51905f52602085015b8383106112135750505050565b60016020819260405161122581610f5a565b8554848060a01b038116825260a01c83820152815201920192019190611206565b805182101561125a5760209160051b010190565b634e487b7160e01b5f52603260045260245ffd5b91908201809211610ad657565b6112836111cf565b805190925f6112aa61129484610f96565b936112a26040519586610f75565b808552610f96565b6020840190601f19013682375f5b86518110156112ea576001906001600160a01b036112d6828a611246565b5151166112e38288611246565b52016112b8565b5060408051639004134760e01b81526001600160a01b0390961660048701526024860152925160448501819052919390929091829160648301915f5b81811061147757505f939283900391508290507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa908115610ac7575f916113e7575b505f915b84518310156113c95761138a8383611246565b51906001600160601b0360206113a08689611246565b5101511691828102928184041490151715610ad6576001916113c19161126e565b920191611377565b915050612710919250046067548110155f146113e25790565b505f90565b90503d805f833e6113f88183610f75565b810190602081830312610ad2578051906001600160401b038211610ad257019080601f83011215610ad257815161142e81610f96565b9261143c6040519485610f75565b81845260208085019260051b820101928311610ad257602001905b828210611467575050505f611373565b8151815260209182019101611457565b82516001600160a01b0316845285945060209384019390920191600101611326565b9190915f8382019384129112908015821691151617610ad657565b905f805b83518210156114f2576001906114ea906114e46001600160a01b036114dd8689611246565b5116611c03565b90611499565b9101906114b8565b90506114ff919250611e00565b5050565b805482101561125a575f5260205f2001905f90565b438110156115bd57606b54905f905b8282106115645750508061154457505f5b6001600160e01b031690565b5f198101908111610ad65761155a90606b611503565b505460201c611538565b909161157781841860011c82851661126e565b908263ffffffff61158984606b611503565b505416111561159b5750915b90611527565b9250600181018091111561159557634e487b7160e01b5f52601160045260245ffd5b606460405162461bcd60e51b815260206004820152602060248201527f436865636b706f696e74733a20626c6f636b206e6f7420796574206d696e65646044820152fd5b438110156115bd57606c54905f905b8282106116395750508061162357505f90565b5f198101908111610ad65761155a90606c611503565b909161164c81841860011c82851661126e565b908263ffffffff61165e84606c611503565b50541611156116705750915b90611610565b9250600181018091111561166a57634e487b7160e01b5f52601160045260245ffd5b90438110156115bd578154905f905b8282106116ca575050806116b657505f919050565b5f198101908111610ad65761155a91611503565b90916116dd81841860011c82851661126e565b908263ffffffff6116ee8488611503565b50541611156117005750915b906116a1565b925060018101809111156116fa57634e487b7160e01b5f52601160045260245ffd5b606b548061154457505f90565b8054806116b657505f919050565b805180835260209291819084018484015e5f828201840152601f01601f1916010190565b6033546001600160a01b0316330361177557565b606460405162461bcd60e51b815260206004820152602060248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152fd5b606c54906117c7606c61172f565b908215158061194b575b1561183f576117df816121a9565b915f198401938411610ad65761183560209361181d7f9324f7e5a7c0288808a634ccde44b8e979676474b22e29ee9dd569b55e791a4b96606c611503565b509063ffffffff82549181199060201b169116179055565b50604051908152a1565b63ffffffff43116118f757611853816121a9565b916040519361186185610f5a565b4363ffffffff1685526001600160e01b039093166020850190815292600160401b811015610af95780600161189b9201606c55606c611503565b9490946118e45751925163ffffffff90931663ffffffff19602094851b16179093557f9324f7e5a7c0288808a634ccde44b8e979676474b22e29ee9dd569b55e791a4b92611835565b634e487b7160e01b5f525f60045260245ffd5b60405162461bcd60e51b815260206004820152602660248201527f53616665436173743a2076616c756520646f65736e27742066697420696e203360448201526532206269747360d01b6064820152608490fd5b505f198301838111610ad65761196763ffffffff91606c611503565b50541643146117d1565b603380546001600160a01b039283166001600160a01b0319821681179092559091167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e05f80a3565b6001600160a01b039081165f818152606a60205260409020909291906119de9061172f565b6001600160a01b0390921692911690828214611a3957805f52606a602052611a098360405f20611f2e565b50506040519182527fd061168252f441733658f09e4d8f5b2d998ed4ef24a2bbfd6ceca52ea131500260204393a4565b505050565b15611a4557565b60405162461bcd60e51b815260206004820152602b60248201527f496e697469616c697a61626c653a20636f6e7472616374206973206e6f74206960448201526a6e697469616c697a696e6760a81b6064820152608490fd5b611aa781612017565b15611bdc5760405191611ab983610f3f565b611ac16111cf565b83526066545f60665580611ba2575b505f5b82518051821015611b4f5781611ae891611246565b519060665491600160401b831015610af9576001830160665560665483101561125a5760665f5280516020919091015160a01b6001600160a01b0319166001600160a01b0391909116175f5160206123295f395f51905f5290920191909155600101611ad3565b505091611b8f90611b9d7f23aad4e61744ece164130aa415c1616e80136b0f0770e56589438b90b269265e936040519384936040855260408501906110a5565b9083820360208501526110a5565b0390a1565b60665f525f5160206123295f395f51905f52015f5160206123295f395f51905f525b818110611bd15750611ad0565b5f8155600101611bc4565b63d173577960e01b5f5260045ffd5b81810392915f138015828513169184121617610ad657565b6001600160a01b0381165f818152606d602052604081209092908390611c289061172f565b8094845f52606e60205260ff60405f205416155f14611da257611c4c929350611beb565b928315611d9c57825f52606d60205260405f2090815492611c6c8361172f565b84151580611d77575b15611cd057611c835f6121a9565b945f198101908111610ad6577f88770dc862e47a7ed586907857eb1b75e4c5ffc8b707c7ee10eb74d6885fe5949561181d611cc092604097611503565b505b82519182526020820152a290565b63ffffffff43116118f757611ce45f6121a9565b9360405195611cf287610f5a565b4363ffffffff1687526001600160e01b039095166020870190815294600160401b811015610af957611d2991600182018155611503565b9590956118e45751935160201b63ffffffff191663ffffffff94909416939093179093557f88770dc862e47a7ed586907857eb1b75e4c5ffc8b707c7ee10eb74d6885fe59492604092611cc0565b505f198501858111610ad657611d9263ffffffff9186611503565b5054164314611c75565b50505090565b505050611dae9061127b565b91611db98184611beb565b928315611d9c577f88770dc862e47a7ed586907857eb1b75e4c5ffc8b707c7ee10eb74d6885fe59491604091845f52606d602052611df981845f20611f2e565b5050611cc2565b90611e13611e0c611722565b9283611499565b90606b54611e21606b61172f565b9080151580611f08575b15611e8457611e39846121a9565b5f198201918211610ad65761181d611e5292606b611503565b507f86dcf86b12dfeedea74ae9300dbdaa193bcce5809369c8177ea2f4eaaa65729b60408051858152846020820152a1565b63ffffffff43116118f757611e98846121a9565b60405191611ea583610f5a565b4363ffffffff1683526001600160e01b039091166020830190815290600160401b811015610af957806001611edf9201606b55606b611503565b9190916118e4579151915160201b63ffffffff191663ffffffff92909216919091179055611e52565b505f198101818111610ad657611f2463ffffffff91606b611503565b5054164314611e2b565b9190918054611f3c8261172f565b9181151580611ff2575b15611f7157611f54856121a9565b905f198301928311610ad657611f6d9261181d91611503565b9190565b63ffffffff43116118f757611f85856121a9565b9060405192611f9384610f5a565b4363ffffffff1684526001600160e01b039092166020840190815291600160401b811015610af957611fca91600182018155611503565b9190916118e4579151915160201b63ffffffff191663ffffffff929092169190911790559190565b505f198201828111610ad65761200d63ffffffff9183611503565b5054164314611f46565b51905f80805b845182101561207a576001600160a01b036120388387611246565b515116906001600160a01b0316811115610e2a57612071600191936001600160601b036020612067868a611246565b510151169061126e565b9101909161201d565b5091925061271014905061208c575f90565b600190565b91909161209e8284612218565b60058110156121955715908161217f575b50612177575f926120e96120f785946040519283916020830195630b135d3f60e11b8752602484015260406044840152606483019061173d565b03601f198101835282610f75565b51915afa3d15612170573d61210b81611044565b906121196040519283610f75565b81523d5f602083013e5b81612164575b81612132575090565b9050602081805181010312610ad257602001516001600160e01b0319811690819003610ad257630b135d3f60e11b1490565b80516020149150612129565b6060612123565b505050600190565b6001600160a01b0383811691161490505f6120af565b634e487b7160e01b5f52602160045260245ffd5b6001600160e01b0381116121c3576001600160e01b031690565b60405162461bcd60e51b815260206004820152602760248201527f53616665436173743a2076616c756520646f65736e27742066697420696e20326044820152663234206269747360c81b6064820152608490fd5b81516041810361224457509061224091602082015190606060408401519301515f1a90612286565b9091565b60400361227d5760406020830151920151918260ff1c91601b8301809311610ad657612240936001600160ff1b03169260ff1690612286565b50505f90600290565b907f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a0841161231d5760ff1690601b82141580612312575b612307576020935f93608093604051938452868401526040830152606082015282805260015afa15610ac7575f516001600160a01b038116156122ff57905f90565b505f90600190565b505050505f90600490565b50601c8214156122bd565b505050505f9060039056fe46501879b8ca8525e8c2fd519e2fbfcfa2ebea26501294aa02cbfcfb12e94354a2646970667358221220d47de43e6e0e951a74532099040e816f8134af5294bff6965b453b994451762964736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10\x15a\0\x11W_\x80\xFD[__5`\xE0\x1C\x80b\xCF*\xB5\x14a\x0F\nW\x80c\r\xBA3\x94\x14a\x0E\xE1W\x80c\x16&\xBA~\x14a\x0B\xD7W\x80c\x17\x03\xA0\x18\x14a\x0B\x87W\x80c\x1EL\xD8^\x14a\x0B^W\x80c1O:I\x14a\x0BDW\x80c;$.J\x14a\x0B\rW\x80c=V\x11\xF6\x14a\t$W\x80c@\xBF/\xB7\x14a\t\x06W\x80cQ@\xA5H\x14a\x07\xFAW\x80c^\x10B\xE8\x14a\x07\xB4W\x80c^\xF53)\x14a\x07\x8FW\x80cibU\xBE\x14a\x07\x14W\x80cqP\x18\xA6\x14a\x06\xB7W\x80ct<1\xF4\x14a\x06oW\x80c\x85}\xC1\x90\x14a\x05ZW\x80c\x8D\xA5\xCB[\x14a\x051W\x80c\x95_-\x90\x14a\x04\xE4W\x80c\x98\xEC\x1A\xC9\x14a\x04\xB8W\x80c\xA2\xCE_\xD1\x14a\x04mW\x80c\xAB\x11\x89\x95\x14a\x02\xD8W\x80c\xB93\xFAt\x14a\x02pW\x80c\xDE\xC5\xD1\xF6\x14a\x01\xFEW\x80c\xEC\x7F\xBB1\x14a\x01\xBFWc\xF2\xFD\xE3\x8B\x14a\x01,W_\x80\xFD[4a\x01\xBCW` 6`\x03\x19\x01\x12a\x01\xBCWa\x01Ea\x0F\xADV[a\x01Ma\x17aV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a\x01hWa\x01e\x90a\x19qV[\x80\xF3[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x90\xFD[\x80\xFD[P4a\x01\xBCW` 6`\x03\x19\x01\x12a\x01\xBCW` \x90`\xFF\x90`@\x90`\x01`\x01`\xA0\x1B\x03a\x01\xEAa\x0F\xADV[\x16\x81R`n\x84R T\x16`@Q\x90\x15\x15\x81R\xF3[P4a\x01\xBCW`@6`\x03\x19\x01\x12a\x01\xBCW`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x02lWa\x02/\x906\x90`\x04\x01a\x10\xFEV[`$5\x90`\x01`\x01`@\x1B\x03\x82\x11a\x02hWa\x02ca\x02Ua\x01e\x936\x90`\x04\x01a\x0F\xC3V[\x91a\x02^a\x17aV[a\x1A\x9EV[a\x14\xB4V[\x82\x80\xFD[P\x80\xFD[P4a\x01\xBCW\x80`\x03\x196\x01\x12a\x01\xBCW`lT\x90\x81a\x02\xA2W` \x91P[`@Q`\x01`\x01`\xE0\x1B\x03\x91\x90\x91\x16\x81R\xF3[_\x19\x82\x01\x91\x82\x11a\x02\xC4WPa\x02\xBB` \x91`la\x15\x03V[PT\x81\x1Ca\x02\x8FV[cNH{q`\xE0\x1B\x81R`\x11`\x04R`$\x90\xFD[P4a\x01\xBCW``6`\x03\x19\x01\x12a\x01\xBCWa\x02\xF2a\x0F\xADV[`D5`\x01`\x01`@\x1B\x03\x81\x11a\x02hWa\x03\x11\x906\x90`\x04\x01a\x10\xFEV[\x82T\x90`\xFF\x82`\x08\x1C\x16\x15\x92\x83\x80\x94a\x04`W[\x80\x15a\x04IW[\x15a\x03\xEDW`\xFF\x19\x83\x16`\x01\x17\x85Ua\x03|\x92\x84a\x03\xDCW[Pa\x03V`\xFF\x86T`\x08\x1C\x16a\x1A>V[`\x01\x80`\xA0\x1B\x03\x16`\x01`\x01``\x1B\x03`\xA0\x1B`hT\x16\x17`hUa\x02^`$5a\x17\xB9V[a\x03\x95`\xFF\x83T`\x08\x1C\x16a\x03\x90\x81a\x1A>V[a\x1A>V[a\x03\x9E3a\x19qV[a\x03\xA5W\x80\xF3[a\xFF\0\x19\x81T\x16\x81U\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98` `@Q`\x01\x81R\xA1\x80\xF3[a\xFF\xFF\x19\x16a\x01\x01\x17\x85U_a\x03EV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x90\xFD[P0;\x15\x80\x15a\x03,WP`\x01`\xFF\x84\x16\x14a\x03,V[P`\x01`\xFF\x84\x16\x10a\x03%V[P4a\x01\xBCW` 6`\x03\x19\x01\x12a\x01\xBCW` \x90`\x01`\x01`\xA0\x1B\x03a\x04\x92a\x0F\xADV[\x16\x81R`j\x82R`@\x90 `\x01`\x01`\xA0\x1B\x03\x90a\x04\xAF\x90a\x17/V[\x16`@Q\x90\x81R\xF3[P4a\x01\xBCW` 6`\x03\x19\x01\x12a\x01\xBCW` a\x04\xDCa\x04\xD7a\x0F\xADV[a\x12{V[`@Q\x90\x81R\xF3[P4a\x01\xBCW`@6`\x03\x19\x01\x12a\x01\xBCWa\x04\xFEa\x0F\xADV[`$5\x90c\xFF\xFF\xFF\xFF\x82\x16\x80\x92\x03a\x02hW`\x01`\x01`\xA0\x1B\x03\x16\x82R`m` \x90\x81R`@\x90\x92 a\x04\xDC\x91\x90a\x16\x92V[P4a\x01\xBCW\x80`\x03\x196\x01\x12a\x01\xBCW`3T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[P4a\x01\xBCW\x80`\x03\x196\x01\x12a\x01\xBCW3\x81R`n` R`\xFF`@\x82 T\x16\x15a\x06`W`eT\x80\x15a\x06LW_\x19\x01`eU3\x81R`n` R`@\x81 `\xFF\x19\x81T\x16\x90Ua\x05\xB4a\x05\xAF3a\x1C\x03V[a\x1E\0V[PP`hT\x81\x90`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x06IW\x81\x80\x91`$`@Q\x80\x94\x81\x93cQ\xB2zm`\xE1\x1B\x83R3`\x04\x84\x01RZ\xF1\x80\x15a\x06>Wa\x06)W[P`hT`\x01`\x01`\xA0\x1B\x03\x163\x7F1\xE0\xAD\xFE\xC7\x1B\xCC\xEE7\xB6\xE8:\x90\xC2\xFE\xDB\x17\xD8\xF1i?\xEE\x86<Gq\xE7\xBF\xE2\xAE\xD5\x80\x83\x80\xA3\x80\xF3[\x81a\x063\x91a\x0FuV[a\x01\xBCW\x80_a\x05\xF4V[`@Q=\x84\x82>=\x90\xFD[P\xFD[cNH{q`\xE0\x1B\x82R`\x11`\x04R`$\x82\xFD[c%\xECl\x1F`\xE0\x1B\x81R`\x04\x90\xFD[P4a\x01\xBCW` 6`\x03\x19\x01\x12a\x01\xBCWa\x06\x89a\x0F\xADV[3\x82R`n` R`\xFF`@\x83 T\x16\x15a\x06\xA8Wa\x01e\x903a\x19\xB9V[c%\xECl\x1F`\xE0\x1B\x82R`\x04\x82\xFD[P4a\x01\xBCW\x80`\x03\x196\x01\x12a\x01\xBCWa\x06\xD0a\x17aV[`3\x80T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16\x90\x91U\x81\x90`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x82\x80\xA3\x80\xF3[P4a\x01\xBCW`@6`\x03\x19\x01\x12a\x01\xBCW`$5`\x045`\x01`\x01`@\x1B\x03\x82\x11a\x02hW\x7Fq<\xA5;\x88\xD6\xEBc\xF5\xB1\x85L\xB8\xCB\xDDsn\xC5\x1E\xDA\"^Fy\x1A\xA9)\x8B\x01`d\x8F`@a\x07na\x01e\x946\x90`\x04\x01a\x0F\xC3V[\x92a\x07wa\x17aV[`gT\x90\x80`gU\x82Q\x91\x82R` \x82\x01R\xA1a\x14\xB4V[P4a\x01\xBCW` 6`\x03\x19\x01\x12a\x01\xBCWa\x07\xA9a\x17aV[a\x01e`\x045a\x17\xB9V[P4a\x01\xBCW`@6`\x03\x19\x01\x12a\x01\xBCW` \x90`\x01`\x01`\xA0\x1B\x03a\x07\xD9a\x0F\xADV[\x16\x81R`j\x82R`@\x90 `\x01`\x01`\xA0\x1B\x03\x90a\x04\xAF\x90`$5\x90a\x16\x92V[P4a\x01\xBCW`@6`\x03\x19\x01\x12a\x01\xBCW`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x02lW6`#\x82\x01\x12\x15a\x02lW\x80`\x04\x015\x90a\x087\x82a\x0F\x96V[\x90a\x08E`@Q\x92\x83a\x0FuV[\x82\x82R` \x82\x01\x92`\x05\x1B\x81\x01`$\x01\x836\x82\x11a\t\x02W`$\x83\x01\x90[\x82\x82\x10a\x08\xCFWPPPP`$5`\x01`\x01`@\x1B\x03\x81\x11a\x08\xCBWa\x08\x8D\x906\x90`\x04\x01a\x10_V[PQ\x15a\x08\xB7WQ\x80Q`eT\x03a\x08\xA8Wa\x01e\x90a\x14\xB4V[c\x16\x9E\xFB[`\xE1\x1B\x82R`\x04\x82\xFD[cNH{q`\xE0\x1B\x82R`2`\x04R`$\x82\xFD[\x83\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11a\x08\xFEW` \x91a\x08\xF3\x83\x92`$6\x91\x89\x01\x01a\x0F\xC3V[\x81R\x01\x91\x01\x90a\x08cV[\x87\x80\xFD[\x85\x80\xFD[P4a\x01\xBCW\x80`\x03\x196\x01\x12a\x01\xBCW` `gT`@Q\x90\x81R\xF3[P4a\n\xD2W`@6`\x03\x19\x01\x12a\n\xD2W`\x045`\x01`\x01`@\x1B\x03\x81\x11a\n\xD2W```\x03\x19\x826\x03\x01\x12a\n\xD2W`@Q``\x81\x01\x81\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\n\xF9W`@R\x81`\x04\x015`\x01`\x01`@\x1B\x03\x81\x11a\n\xD2Wa\t\x94\x90`\x046\x91\x85\x01\x01a\x10_V[\x81R`$\x80\x83\x015` \x83\x01\x90\x81R`D\x90\x93\x015`@\x83\x01\x90\x81R\x905`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\n\xD2W3_R`n` R`\xFF`@_ T\x16a\n\xEAW`eT\x90_\x19\x82\x14a\n\xD6W`\x01a\n\x17\x92\x01`eU3_R`n` R`@_ `\x01`\xFF\x19\x82T\x16\x17\x90Ua\n\x0Fa\x05\xAF3a\x1C\x03V[PP3a\x19\xB9V[`hT`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\n\xD2Wa\nc_\x80\x94`@Q\x96\x87\x95\x86\x94\x85\x93c\x99&\xEE}`\xE0\x1B\x85R3`\x04\x86\x01R`@`$\x86\x01RQ```D\x86\x01R`\xA4\x85\x01\x90a\x17=V[\x91Q`d\x84\x01RQ`\x84\x83\x01R\x03\x92Z\xF1\x80\x15a\n\xC7Wa\n\xB4W[P`hT`\x01`\x01`\xA0\x1B\x03\x163\x7F\xA4S\xDBa*\xF5\x9EU!\xD6\xAB\x92\x84\xDC>-\x06\xAF(n\xB1\xB1\xB7\xB7q\xFC\xE4ql\x19\xF2\xC1\x83\x80\xA3\x80\xF3[a\n\xC0\x91P_\x90a\x0FuV[__a\n\x7FV[`@Q=_\x82>=\x90\xFD[_\x80\xFD[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[cB\xEEh\xB5`\xE0\x1B_R`\x04_\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[4a\n\xD2W` 6`\x03\x19\x01\x12a\n\xD2W`\x01`\x01`\xA0\x1B\x03a\x0B.a\x0F\xADV[\x16_R`m` R` a\x04\xDC`@_ a\x17/V[4a\n\xD2W_6`\x03\x19\x01\x12a\n\xD2W` a\x04\xDCa\x17\"V[4a\n\xD2W` 6`\x03\x19\x01\x12a\n\xD2W` a\x04\xDCc\xFF\xFF\xFF\xFFa\x0B\x81a\x101V[\x16a\x16\x01V[4a\n\xD2W_6`\x03\x19\x01\x12a\n\xD2W```@Qa\x0B\xA5\x81a\x0F?V[Ra\x0B\xD3`@Qa\x0B\xB5\x81a\x0F?V[a\x0B\xBDa\x11\xCFV[\x81R`@Q\x91\x82\x91` \x83R` \x83\x01\x90a\x10\xA5V[\x03\x90\xF3[4a\n\xD2W`@6`\x03\x19\x01\x12a\n\xD2W`\x045`$5`\x01`\x01`@\x1B\x03\x81\x11a\n\xD2Wa\x0C\n\x906\x90`\x04\x01a\x10_V[\x80Q\x81\x01``\x82` \x83\x01\x92\x03\x12a\n\xD2W` \x82\x01Q`\x01`\x01`@\x1B\x03\x81\x11a\n\xD2W\x82\x01\x91\x81`?\x84\x01\x12\x15a\n\xD2W` \x83\x01Qa\x0CK\x81a\x0F\x96V[\x93a\x0CY`@Q\x95\x86a\x0FuV[\x81\x85R` \x80\x80\x87\x01\x93`\x05\x1B\x83\x01\x01\x01\x90\x84\x82\x11a\n\xD2W`@\x01\x91[\x81\x83\x10a\x0E\xC1WPPP`@\x81\x01Q`\x01`\x01`@\x1B\x03\x81\x11a\n\xD2W\x81\x01\x91\x80`?\x84\x01\x12\x15a\n\xD2W` \x83\x01Q\x92a\x0C\xB1\x84a\x0F\x96V[\x93a\x0C\xBF`@Q\x95\x86a\x0FuV[\x80\x85R` \x80\x80\x87\x01\x92`\x05\x1B\x84\x01\x01\x01\x91\x83\x83\x11a\n\xD2W`@\x81\x01\x91[\x83\x83\x10a\x0EWW\x88\x88\x88``\x89\x01Q\x91c\xFF\xFF\xFF\xFF\x83\x16\x80\x93\x03a\n\xD2W\x80Q\x91__\x95\x82Q\x85\x03a\x0EHW\x84\x15a\x0E9WC\x86\x10\x15\x94_\x94\x90\x93\x92[\x84\x86\x10a\r\x85W\x88\x88\x88a\rvWa\r2\x81a\x15\x18V[\x82\x11a\rgWa\rA\x90a\x16\x01V[\x11a\rXW`@Qc\x0B\x13]?`\xE1\x1B\x81R` \x90\xF3[c\xE1!c/`\xE0\x1B_R`\x04_\xFD[cK\x05\xA0\xF7`\xE1\x1B_R`\x04_\xFD[c\xE6O\x18\x0F`\xE0\x1B_R`\x04_\xFD[\x94\x97\x93\x94\x92\x93\x92`\x01`\x01`\xA0\x1B\x03a\r\x9E\x8A\x84a\x12FV[Q\x16\x96a\rvW\x86_R`j` R\x86`\x01\x80`\xA0\x1B\x03a\r\xC2\x8A`@_ a\x16\x92V[\x16\x91`\x01`\x01`\xA0\x1B\x03\x16\x10\x15a\x0E*Wa\r\xE9\x90\x83a\r\xE2\x8B\x88a\x12FV[Q\x91a \x91V[\x15a\x0E\x1BW`\x01a\x0E\x11\x87\x94_\x98_R`m` Ra\x0E\x0B\x8A`@_ a\x16\x92V[\x90a\x12nV[\x98\x01\x94\x93\x92a\r\x1BV[c\x8B\xAAW\x9F`\xE0\x1B_R`\x04_\xFD[c\xBAP\xF9\x11`\xE0\x1B_R`\x04_\xFD[c%\x1FV\xA1`\xE2\x1B_R`\x04_\xFD[c\x1F\xECgG`\xE3\x1B_R`\x04_\xFD[\x82Q`\x01`\x01`@\x1B\x03\x81\x11a\n\xD2W` \x90\x83\x01\x01\x85`?\x82\x01\x12\x15a\n\xD2W` \x81\x01Q\x91a\x0E\x87\x83a\x10DV[a\x0E\x94`@Q\x91\x82a\x0FuV[\x83\x81R`@\x83\x85\x01\x01\x88\x10a\n\xD2W_` \x85\x81\x96`@\x83\x97\x01\x83\x86\x01^\x83\x01\x01R\x81R\x01\x92\x01\x91a\x0C\xDEV[\x82Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\n\xD2W\x81R` \x92\x83\x01\x92\x01a\x0CwV[4a\n\xD2W` 6`\x03\x19\x01\x12a\n\xD2W` a\x04\xDCc\xFF\xFF\xFF\xFFa\x0F\x04a\x101V[\x16a\x15\x18V[4a\n\xD2W` 6`\x03\x19\x01\x12a\n\xD2W`\x045`\x01`\x01`@\x1B\x03\x81\x11a\n\xD2Wa\x02ca\x0F=\x916\x90`\x04\x01a\x0F\xC3V[\0[` \x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\n\xF9W`@RV[`@\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\n\xF9W`@RV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\n\xF9W`@RV[`\x01`\x01`@\x1B\x03\x81\x11a\n\xF9W`\x05\x1B` \x01\x90V[`\x045\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\n\xD2WV[\x90\x80`\x1F\x83\x01\x12\x15a\n\xD2W\x815\x90a\x0F\xDB\x82a\x0F\x96V[\x92a\x0F\xE9`@Q\x94\x85a\x0FuV[\x82\x84R` \x80\x85\x01\x93`\x05\x1B\x82\x01\x01\x91\x82\x11a\n\xD2W` \x01\x91[\x81\x83\x10a\x10\x11WPPP\x90V[\x825`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\n\xD2W\x81R` \x92\x83\x01\x92\x01a\x10\x04V[`\x045\x90c\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\n\xD2WV[`\x01`\x01`@\x1B\x03\x81\x11a\n\xF9W`\x1F\x01`\x1F\x19\x16` \x01\x90V[\x81`\x1F\x82\x01\x12\x15a\n\xD2W\x805\x90a\x10v\x82a\x10DV[\x92a\x10\x84`@Q\x94\x85a\x0FuV[\x82\x84R` \x83\x83\x01\x01\x11a\n\xD2W\x81_\x92` \x80\x93\x01\x83\x86\x017\x83\x01\x01R\x90V[` `@\x81\x84\x01\x92Q\x93\x82\x81R\x84Q\x80\x94R\x01\x92\x01\x90_[\x81\x81\x10a\x10\xCAWPPP\x90V[\x82Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x85R` \x90\x81\x01Q`\x01`\x01``\x1B\x03\x16\x81\x86\x01R`@\x90\x94\x01\x93\x90\x92\x01\x91`\x01\x01a\x10\xBDV[\x91\x90` \x83\x82\x03\x12a\n\xD2W`@Qa\x11\x16\x81a\x0F?V[\x80\x93\x805\x90`\x01`\x01`@\x1B\x03\x82\x11a\n\xD2W\x01\x82`\x1F\x82\x01\x12\x15a\n\xD2W\x805\x90a\x11A\x82a\x0F\x96V[\x93a\x11O`@Q\x95\x86a\x0FuV[\x82\x85R` \x80\x86\x01\x93`\x06\x1B\x83\x01\x01\x91\x81\x83\x11a\n\xD2W` \x01\x92[\x82\x84\x10a\x11yWPPPPRV[`@\x84\x83\x03\x12a\n\xD2W`@Q\x90a\x11\x90\x82a\x0FZV[\x845`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\n\xD2W\x82R` \x85\x015\x90`\x01`\x01``\x1B\x03\x82\x16\x82\x03a\n\xD2W\x82` \x92\x83`@\x95\x01R\x81R\x01\x93\x01\x92a\x11kV[`fT\x90a\x11\xDC\x82a\x0F\x96V[\x91a\x11\xEA`@Q\x93\x84a\x0FuV[\x80\x83R`f_\x90\x81R_Q` a#)_9_Q\x90_R` \x85\x01[\x83\x83\x10a\x12\x13WPPPPV[`\x01` \x81\x92`@Qa\x12%\x81a\x0FZV[\x85T\x84\x80`\xA0\x1B\x03\x81\x16\x82R`\xA0\x1C\x83\x82\x01R\x81R\x01\x92\x01\x92\x01\x91\x90a\x12\x06V[\x80Q\x82\x10\x15a\x12ZW` \x91`\x05\x1B\x01\x01\x90V[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[\x91\x90\x82\x01\x80\x92\x11a\n\xD6WV[a\x12\x83a\x11\xCFV[\x80Q\x90\x92_a\x12\xAAa\x12\x94\x84a\x0F\x96V[\x93a\x12\xA2`@Q\x95\x86a\x0FuV[\x80\x85Ra\x0F\x96V[` \x84\x01\x90`\x1F\x19\x016\x827_[\x86Q\x81\x10\x15a\x12\xEAW`\x01\x90`\x01`\x01`\xA0\x1B\x03a\x12\xD6\x82\x8Aa\x12FV[QQ\x16a\x12\xE3\x82\x88a\x12FV[R\x01a\x12\xB8V[P`@\x80Qc\x90\x04\x13G`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x96\x16`\x04\x87\x01R`$\x86\x01R\x92Q`D\x85\x01\x81\x90R\x91\x93\x90\x92\x90\x91\x82\x91`d\x83\x01\x91_[\x81\x81\x10a\x14wWP_\x93\x92\x83\x90\x03\x91P\x82\x90P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\n\xC7W_\x91a\x13\xE7W[P_\x91[\x84Q\x83\x10\x15a\x13\xC9Wa\x13\x8A\x83\x83a\x12FV[Q\x90`\x01`\x01``\x1B\x03` a\x13\xA0\x86\x89a\x12FV[Q\x01Q\x16\x91\x82\x81\x02\x92\x81\x84\x04\x14\x90\x15\x17\x15a\n\xD6W`\x01\x91a\x13\xC1\x91a\x12nV[\x92\x01\x91a\x13wV[\x91PPa'\x10\x91\x92P\x04`gT\x81\x10\x15_\x14a\x13\xE2W\x90V[P_\x90V[\x90P=\x80_\x83>a\x13\xF8\x81\x83a\x0FuV[\x81\x01\x90` \x81\x83\x03\x12a\n\xD2W\x80Q\x90`\x01`\x01`@\x1B\x03\x82\x11a\n\xD2W\x01\x90\x80`\x1F\x83\x01\x12\x15a\n\xD2W\x81Qa\x14.\x81a\x0F\x96V[\x92a\x14<`@Q\x94\x85a\x0FuV[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\n\xD2W` \x01\x90[\x82\x82\x10a\x14gWPPP_a\x13sV[\x81Q\x81R` \x91\x82\x01\x91\x01a\x14WV[\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x84R\x85\x94P` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x13&V[\x91\x90\x91_\x83\x82\x01\x93\x84\x12\x91\x12\x90\x80\x15\x82\x16\x91\x15\x16\x17a\n\xD6WV[\x90_\x80[\x83Q\x82\x10\x15a\x14\xF2W`\x01\x90a\x14\xEA\x90a\x14\xE4`\x01`\x01`\xA0\x1B\x03a\x14\xDD\x86\x89a\x12FV[Q\x16a\x1C\x03V[\x90a\x14\x99V[\x91\x01\x90a\x14\xB8V[\x90Pa\x14\xFF\x91\x92Pa\x1E\0V[PPV[\x80T\x82\x10\x15a\x12ZW_R` _ \x01\x90_\x90V[C\x81\x10\x15a\x15\xBDW`kT\x90_\x90[\x82\x82\x10a\x15dWPP\x80a\x15DWP_[`\x01`\x01`\xE0\x1B\x03\x16\x90V[_\x19\x81\x01\x90\x81\x11a\n\xD6Wa\x15Z\x90`ka\x15\x03V[PT` \x1Ca\x158V[\x90\x91a\x15w\x81\x84\x18`\x01\x1C\x82\x85\x16a\x12nV[\x90\x82c\xFF\xFF\xFF\xFFa\x15\x89\x84`ka\x15\x03V[PT\x16\x11\x15a\x15\x9BWP\x91[\x90a\x15'V[\x92P`\x01\x81\x01\x80\x91\x11\x15a\x15\x95WcNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[`d`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R` `$\x82\x01R\x7FCheckpoints: block not yet mined`D\x82\x01R\xFD[C\x81\x10\x15a\x15\xBDW`lT\x90_\x90[\x82\x82\x10a\x169WPP\x80a\x16#WP_\x90V[_\x19\x81\x01\x90\x81\x11a\n\xD6Wa\x15Z\x90`la\x15\x03V[\x90\x91a\x16L\x81\x84\x18`\x01\x1C\x82\x85\x16a\x12nV[\x90\x82c\xFF\xFF\xFF\xFFa\x16^\x84`la\x15\x03V[PT\x16\x11\x15a\x16pWP\x91[\x90a\x16\x10V[\x92P`\x01\x81\x01\x80\x91\x11\x15a\x16jWcNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x90C\x81\x10\x15a\x15\xBDW\x81T\x90_\x90[\x82\x82\x10a\x16\xCAWPP\x80a\x16\xB6WP_\x91\x90PV[_\x19\x81\x01\x90\x81\x11a\n\xD6Wa\x15Z\x91a\x15\x03V[\x90\x91a\x16\xDD\x81\x84\x18`\x01\x1C\x82\x85\x16a\x12nV[\x90\x82c\xFF\xFF\xFF\xFFa\x16\xEE\x84\x88a\x15\x03V[PT\x16\x11\x15a\x17\0WP\x91[\x90a\x16\xA1V[\x92P`\x01\x81\x01\x80\x91\x11\x15a\x16\xFAWcNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[`kT\x80a\x15DWP_\x90V[\x80T\x80a\x16\xB6WP_\x91\x90PV[\x80Q\x80\x83R` \x92\x91\x81\x90\x84\x01\x84\x84\x01^_\x82\x82\x01\x84\x01R`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[`3T`\x01`\x01`\xA0\x1B\x03\x163\x03a\x17uWV[`d`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R` `$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R\xFD[`lT\x90a\x17\xC7`la\x17/V[\x90\x82\x15\x15\x80a\x19KW[\x15a\x18?Wa\x17\xDF\x81a!\xA9V[\x91_\x19\x84\x01\x93\x84\x11a\n\xD6Wa\x185` \x93a\x18\x1D\x7F\x93$\xF7\xE5\xA7\xC0(\x88\x08\xA64\xCC\xDED\xB8\xE9ygdt\xB2.)\xEE\x9D\xD5i\xB5^y\x1AK\x96`la\x15\x03V[P\x90c\xFF\xFF\xFF\xFF\x82T\x91\x81\x19\x90` \x1B\x16\x91\x16\x17\x90UV[P`@Q\x90\x81R\xA1V[c\xFF\xFF\xFF\xFFC\x11a\x18\xF7Wa\x18S\x81a!\xA9V[\x91`@Q\x93a\x18a\x85a\x0FZV[Cc\xFF\xFF\xFF\xFF\x16\x85R`\x01`\x01`\xE0\x1B\x03\x90\x93\x16` \x85\x01\x90\x81R\x92`\x01`@\x1B\x81\x10\x15a\n\xF9W\x80`\x01a\x18\x9B\x92\x01`lU`la\x15\x03V[\x94\x90\x94a\x18\xE4WQ\x92Qc\xFF\xFF\xFF\xFF\x90\x93\x16c\xFF\xFF\xFF\xFF\x19` \x94\x85\x1B\x16\x17\x90\x93U\x7F\x93$\xF7\xE5\xA7\xC0(\x88\x08\xA64\xCC\xDED\xB8\xE9ygdt\xB2.)\xEE\x9D\xD5i\xB5^y\x1AK\x92a\x185V[cNH{q`\xE0\x1B_R_`\x04R`$_\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FSafeCast: value doesn't fit in 3`D\x82\x01Re2 bits`\xD0\x1B`d\x82\x01R`\x84\x90\xFD[P_\x19\x83\x01\x83\x81\x11a\n\xD6Wa\x19gc\xFF\xFF\xFF\xFF\x91`la\x15\x03V[PT\x16C\x14a\x17\xD1V[`3\x80T`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x01`\x01`\xA0\x1B\x03\x19\x82\x16\x81\x17\x90\x92U\x90\x91\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0_\x80\xA3V[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16_\x81\x81R`j` R`@\x90 \x90\x92\x91\x90a\x19\xDE\x90a\x17/V[`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x92\x91\x16\x90\x82\x82\x14a\x1A9W\x80_R`j` Ra\x1A\t\x83`@_ a\x1F.V[PP`@Q\x91\x82R\x7F\xD0a\x16\x82R\xF4As6X\xF0\x9EM\x8F[-\x99\x8E\xD4\xEF$\xA2\xBB\xFDl\xEC\xA5.\xA11P\x02` C\x93\xA4V[PPPV[\x15a\x1AEWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x90\xFD[a\x1A\xA7\x81a \x17V[\x15a\x1B\xDCW`@Q\x91a\x1A\xB9\x83a\x0F?V[a\x1A\xC1a\x11\xCFV[\x83R`fT_`fU\x80a\x1B\xA2W[P_[\x82Q\x80Q\x82\x10\x15a\x1BOW\x81a\x1A\xE8\x91a\x12FV[Q\x90`fT\x91`\x01`@\x1B\x83\x10\x15a\n\xF9W`\x01\x83\x01`fU`fT\x83\x10\x15a\x12ZW`f_R\x80Q` \x91\x90\x91\x01Q`\xA0\x1B`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x17_Q` a#)_9_Q\x90_R\x90\x92\x01\x91\x90\x91U`\x01\x01a\x1A\xD3V[PP\x91a\x1B\x8F\x90a\x1B\x9D\x7F#\xAA\xD4\xE6\x17D\xEC\xE1d\x13\n\xA4\x15\xC1an\x80\x13k\x0F\x07p\xE5e\x89C\x8B\x90\xB2i&^\x93`@Q\x93\x84\x93`@\x85R`@\x85\x01\x90a\x10\xA5V[\x90\x83\x82\x03` \x85\x01Ra\x10\xA5V[\x03\x90\xA1V[`f_R_Q` a#)_9_Q\x90_R\x01_Q` a#)_9_Q\x90_R[\x81\x81\x10a\x1B\xD1WPa\x1A\xD0V[_\x81U`\x01\x01a\x1B\xC4V[c\xD1sWy`\xE0\x1B_R`\x04_\xFD[\x81\x81\x03\x92\x91_\x13\x80\x15\x82\x85\x13\x16\x91\x84\x12\x16\x17a\n\xD6WV[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x81\x81R`m` R`@\x81 \x90\x92\x90\x83\x90a\x1C(\x90a\x17/V[\x80\x94\x84_R`n` R`\xFF`@_ T\x16\x15_\x14a\x1D\xA2Wa\x1CL\x92\x93Pa\x1B\xEBV[\x92\x83\x15a\x1D\x9CW\x82_R`m` R`@_ \x90\x81T\x92a\x1Cl\x83a\x17/V[\x84\x15\x15\x80a\x1DwW[\x15a\x1C\xD0Wa\x1C\x83_a!\xA9V[\x94_\x19\x81\x01\x90\x81\x11a\n\xD6W\x7F\x88w\r\xC8b\xE4z~\xD5\x86\x90xW\xEB\x1Bu\xE4\xC5\xFF\xC8\xB7\x07\xC7\xEE\x10\xEBt\xD6\x88_\xE5\x94\x95a\x18\x1Da\x1C\xC0\x92`@\x97a\x15\x03V[P[\x82Q\x91\x82R` \x82\x01R\xA2\x90V[c\xFF\xFF\xFF\xFFC\x11a\x18\xF7Wa\x1C\xE4_a!\xA9V[\x93`@Q\x95a\x1C\xF2\x87a\x0FZV[Cc\xFF\xFF\xFF\xFF\x16\x87R`\x01`\x01`\xE0\x1B\x03\x90\x95\x16` \x87\x01\x90\x81R\x94`\x01`@\x1B\x81\x10\x15a\n\xF9Wa\x1D)\x91`\x01\x82\x01\x81Ua\x15\x03V[\x95\x90\x95a\x18\xE4WQ\x93Q` \x1Bc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x94\x90\x94\x16\x93\x90\x93\x17\x90\x93U\x7F\x88w\r\xC8b\xE4z~\xD5\x86\x90xW\xEB\x1Bu\xE4\xC5\xFF\xC8\xB7\x07\xC7\xEE\x10\xEBt\xD6\x88_\xE5\x94\x92`@\x92a\x1C\xC0V[P_\x19\x85\x01\x85\x81\x11a\n\xD6Wa\x1D\x92c\xFF\xFF\xFF\xFF\x91\x86a\x15\x03V[PT\x16C\x14a\x1CuV[PPP\x90V[PPPa\x1D\xAE\x90a\x12{V[\x91a\x1D\xB9\x81\x84a\x1B\xEBV[\x92\x83\x15a\x1D\x9CW\x7F\x88w\r\xC8b\xE4z~\xD5\x86\x90xW\xEB\x1Bu\xE4\xC5\xFF\xC8\xB7\x07\xC7\xEE\x10\xEBt\xD6\x88_\xE5\x94\x91`@\x91\x84_R`m` Ra\x1D\xF9\x81\x84_ a\x1F.V[PPa\x1C\xC2V[\x90a\x1E\x13a\x1E\x0Ca\x17\"V[\x92\x83a\x14\x99V[\x90`kTa\x1E!`ka\x17/V[\x90\x80\x15\x15\x80a\x1F\x08W[\x15a\x1E\x84Wa\x1E9\x84a!\xA9V[_\x19\x82\x01\x91\x82\x11a\n\xD6Wa\x18\x1Da\x1ER\x92`ka\x15\x03V[P\x7F\x86\xDC\xF8k\x12\xDF\xEE\xDE\xA7J\xE90\r\xBD\xAA\x19;\xCC\xE5\x80\x93i\xC8\x17~\xA2\xF4\xEA\xAAer\x9B`@\x80Q\x85\x81R\x84` \x82\x01R\xA1V[c\xFF\xFF\xFF\xFFC\x11a\x18\xF7Wa\x1E\x98\x84a!\xA9V[`@Q\x91a\x1E\xA5\x83a\x0FZV[Cc\xFF\xFF\xFF\xFF\x16\x83R`\x01`\x01`\xE0\x1B\x03\x90\x91\x16` \x83\x01\x90\x81R\x90`\x01`@\x1B\x81\x10\x15a\n\xF9W\x80`\x01a\x1E\xDF\x92\x01`kU`ka\x15\x03V[\x91\x90\x91a\x18\xE4W\x91Q\x91Q` \x1Bc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90Ua\x1ERV[P_\x19\x81\x01\x81\x81\x11a\n\xD6Wa\x1F$c\xFF\xFF\xFF\xFF\x91`ka\x15\x03V[PT\x16C\x14a\x1E+V[\x91\x90\x91\x80Ta\x1F<\x82a\x17/V[\x91\x81\x15\x15\x80a\x1F\xF2W[\x15a\x1FqWa\x1FT\x85a!\xA9V[\x90_\x19\x83\x01\x92\x83\x11a\n\xD6Wa\x1Fm\x92a\x18\x1D\x91a\x15\x03V[\x91\x90V[c\xFF\xFF\xFF\xFFC\x11a\x18\xF7Wa\x1F\x85\x85a!\xA9V[\x90`@Q\x92a\x1F\x93\x84a\x0FZV[Cc\xFF\xFF\xFF\xFF\x16\x84R`\x01`\x01`\xE0\x1B\x03\x90\x92\x16` \x84\x01\x90\x81R\x91`\x01`@\x1B\x81\x10\x15a\n\xF9Wa\x1F\xCA\x91`\x01\x82\x01\x81Ua\x15\x03V[\x91\x90\x91a\x18\xE4W\x91Q\x91Q` \x1Bc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x91\x90V[P_\x19\x82\x01\x82\x81\x11a\n\xD6Wa \rc\xFF\xFF\xFF\xFF\x91\x83a\x15\x03V[PT\x16C\x14a\x1FFV[Q\x90_\x80\x80[\x84Q\x82\x10\x15a zW`\x01`\x01`\xA0\x1B\x03a 8\x83\x87a\x12FV[QQ\x16\x90`\x01`\x01`\xA0\x1B\x03\x16\x81\x11\x15a\x0E*Wa q`\x01\x91\x93`\x01`\x01``\x1B\x03` a g\x86\x8Aa\x12FV[Q\x01Q\x16\x90a\x12nV[\x91\x01\x90\x91a \x1DV[P\x91\x92Pa'\x10\x14\x90Pa \x8CW_\x90V[`\x01\x90V[\x91\x90\x91a \x9E\x82\x84a\"\x18V[`\x05\x81\x10\x15a!\x95W\x15\x90\x81a!\x7FW[Pa!wW_\x92a \xE9a \xF7\x85\x94`@Q\x92\x83\x91` \x83\x01\x95c\x0B\x13]?`\xE1\x1B\x87R`$\x84\x01R`@`D\x84\x01R`d\x83\x01\x90a\x17=V[\x03`\x1F\x19\x81\x01\x83R\x82a\x0FuV[Q\x91Z\xFA=\x15a!pW=a!\x0B\x81a\x10DV[\x90a!\x19`@Q\x92\x83a\x0FuV[\x81R=_` \x83\x01>[\x81a!dW[\x81a!2WP\x90V[\x90P` \x81\x80Q\x81\x01\x03\x12a\n\xD2W` \x01Q`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x90\x81\x90\x03a\n\xD2Wc\x0B\x13]?`\xE1\x1B\x14\x90V[\x80Q` \x14\x91Pa!)V[``a!#V[PPP`\x01\x90V[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16\x91\x16\x14\x90P_a \xAFV[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[`\x01`\x01`\xE0\x1B\x03\x81\x11a!\xC3W`\x01`\x01`\xE0\x1B\x03\x16\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FSafeCast: value doesn't fit in 2`D\x82\x01Rf24 bits`\xC8\x1B`d\x82\x01R`\x84\x90\xFD[\x81Q`A\x81\x03a\"DWP\x90a\"@\x91` \x82\x01Q\x90```@\x84\x01Q\x93\x01Q_\x1A\x90a\"\x86V[\x90\x91V[`@\x03a\"}W`@` \x83\x01Q\x92\x01Q\x91\x82`\xFF\x1C\x91`\x1B\x83\x01\x80\x93\x11a\n\xD6Wa\"@\x93`\x01`\x01`\xFF\x1B\x03\x16\x92`\xFF\x16\x90a\"\x86V[PP_\x90`\x02\x90V[\x90\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x84\x11a#\x1DW`\xFF\x16\x90`\x1B\x82\x14\x15\x80a#\x12W[a#\x07W` \x93_\x93`\x80\x93`@Q\x93\x84R\x86\x84\x01R`@\x83\x01R``\x82\x01R\x82\x80R`\x01Z\xFA\x15a\n\xC7W_Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a\"\xFFW\x90_\x90V[P_\x90`\x01\x90V[PPPP_\x90`\x04\x90V[P`\x1C\x82\x14\x15a\"\xBDV[PPPP_\x90`\x03\x90V\xFEFP\x18y\xB8\xCA\x85%\xE8\xC2\xFDQ\x9E/\xBF\xCF\xA2\xEB\xEA&P\x12\x94\xAA\x02\xCB\xFC\xFB\x12\xE9CT\xA2dipfsX\"\x12 \xD4}\xE4>n\x0E\x95\x1AtS \x99\x04\x0E\x81o\x814\xAFR\x94\xBF\xF6\x96[E;\x99DQv)dsolcC\0\x08\x1B\x003",
    );
    /**Custom error with signature `InsufficientSignedStake()` and selector `0xe121632f`.
```solidity
error InsufficientSignedStake();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InsufficientSignedStake {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InsufficientSignedStake> for UnderlyingRustTuple<'_> {
            fn from(value: InsufficientSignedStake) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InsufficientSignedStake {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InsufficientSignedStake {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InsufficientSignedStake()";
            const SELECTOR: [u8; 4] = [225u8, 33u8, 99u8, 47u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    /**Custom error with signature `InsufficientWeight()` and selector `0xa8792fd1`.
```solidity
error InsufficientWeight();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InsufficientWeight {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InsufficientWeight> for UnderlyingRustTuple<'_> {
            fn from(value: InsufficientWeight) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InsufficientWeight {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InsufficientWeight {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InsufficientWeight()";
            const SELECTOR: [u8; 4] = [168u8, 121u8, 47u8, 209u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    /**Custom error with signature `InvalidLength()` and selector `0x947d5a84`.
```solidity
error InvalidLength();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidLength {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InvalidLength> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidLength) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidLength {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidLength {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidLength()";
            const SELECTOR: [u8; 4] = [148u8, 125u8, 90u8, 132u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    /**Custom error with signature `InvalidQuorum()` and selector `0xd1735779`.
```solidity
error InvalidQuorum();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidQuorum {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InvalidQuorum> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidQuorum) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidQuorum {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidQuorum {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidQuorum()";
            const SELECTOR: [u8; 4] = [209u8, 115u8, 87u8, 121u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    /**Custom error with signature `InvalidReferenceBlock()` and selector `0xe64f180f`.
```solidity
error InvalidReferenceBlock();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidReferenceBlock {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InvalidReferenceBlock> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidReferenceBlock) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidReferenceBlock {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidReferenceBlock {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidReferenceBlock()";
            const SELECTOR: [u8; 4] = [230u8, 79u8, 24u8, 15u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    /**Custom error with signature `InvalidSignature()` and selector `0x8baa579f`.
```solidity
error InvalidSignature();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidSignature {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InvalidSignature> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidSignature) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidSignature {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidSignature {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidSignature()";
            const SELECTOR: [u8; 4] = [139u8, 170u8, 87u8, 159u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    /**Custom error with signature `InvalidSignedWeight()` and selector `0x960b41ee`.
```solidity
error InvalidSignedWeight();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidSignedWeight {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InvalidSignedWeight> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidSignedWeight) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidSignedWeight {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidSignedWeight {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidSignedWeight()";
            const SELECTOR: [u8; 4] = [150u8, 11u8, 65u8, 238u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    /**Custom error with signature `InvalidThreshold()` and selector `0xaabd5a09`.
```solidity
error InvalidThreshold();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidThreshold {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InvalidThreshold> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidThreshold) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidThreshold {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidThreshold {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidThreshold()";
            const SELECTOR: [u8; 4] = [170u8, 189u8, 90u8, 9u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    /**Custom error with signature `LengthMismatch()` and selector `0xff633a38`.
```solidity
error LengthMismatch();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct LengthMismatch {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<LengthMismatch> for UnderlyingRustTuple<'_> {
            fn from(value: LengthMismatch) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for LengthMismatch {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for LengthMismatch {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "LengthMismatch()";
            const SELECTOR: [u8; 4] = [255u8, 99u8, 58u8, 56u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    /**Custom error with signature `MustUpdateAllOperators()` and selector `0x2d3df6b6`.
```solidity
error MustUpdateAllOperators();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct MustUpdateAllOperators {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<MustUpdateAllOperators> for UnderlyingRustTuple<'_> {
            fn from(value: MustUpdateAllOperators) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for MustUpdateAllOperators {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for MustUpdateAllOperators {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "MustUpdateAllOperators()";
            const SELECTOR: [u8; 4] = [45u8, 61u8, 246u8, 182u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    /**Custom error with signature `NotSorted()` and selector `0xba50f911`.
```solidity
error NotSorted();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NotSorted {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<NotSorted> for UnderlyingRustTuple<'_> {
            fn from(value: NotSorted) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for NotSorted {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for NotSorted {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "NotSorted()";
            const SELECTOR: [u8; 4] = [186u8, 80u8, 249u8, 17u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    /**Custom error with signature `OperatorAlreadyRegistered()` and selector `0x42ee68b5`.
```solidity
error OperatorAlreadyRegistered();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OperatorAlreadyRegistered {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<OperatorAlreadyRegistered>
        for UnderlyingRustTuple<'_> {
            fn from(value: OperatorAlreadyRegistered) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for OperatorAlreadyRegistered {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OperatorAlreadyRegistered {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OperatorAlreadyRegistered()";
            const SELECTOR: [u8; 4] = [66u8, 238u8, 104u8, 181u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    /**Custom error with signature `OperatorNotRegistered()` and selector `0x25ec6c1f`.
```solidity
error OperatorNotRegistered();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OperatorNotRegistered {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<OperatorNotRegistered> for UnderlyingRustTuple<'_> {
            fn from(value: OperatorNotRegistered) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OperatorNotRegistered {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OperatorNotRegistered {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OperatorNotRegistered()";
            const SELECTOR: [u8; 4] = [37u8, 236u8, 108u8, 31u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    /**Event with signature `Initialized(uint8)` and selector `0x7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb3847402498`.
```solidity
event Initialized(uint8 version);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct Initialized {
        #[allow(missing_docs)]
        pub version: u8,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for Initialized {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "Initialized(uint8)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                127u8,
                38u8,
                184u8,
                63u8,
                249u8,
                110u8,
                31u8,
                43u8,
                106u8,
                104u8,
                47u8,
                19u8,
                56u8,
                82u8,
                246u8,
                121u8,
                138u8,
                9u8,
                196u8,
                101u8,
                218u8,
                149u8,
                146u8,
                20u8,
                96u8,
                206u8,
                251u8,
                56u8,
                71u8,
                64u8,
                36u8,
                152u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { version: data.0 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::tokenize(&self.version),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for Initialized {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&Initialized> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &Initialized) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `MinimumWeightUpdated(uint256,uint256)` and selector `0x713ca53b88d6eb63f5b1854cb8cbdd736ec51eda225e46791aa9298b0160648f`.
```solidity
event MinimumWeightUpdated(uint256 previous, uint256 current);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct MinimumWeightUpdated {
        #[allow(missing_docs)]
        pub previous: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub current: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for MinimumWeightUpdated {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "MinimumWeightUpdated(uint256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                113u8,
                60u8,
                165u8,
                59u8,
                136u8,
                214u8,
                235u8,
                99u8,
                245u8,
                177u8,
                133u8,
                76u8,
                184u8,
                203u8,
                221u8,
                115u8,
                110u8,
                197u8,
                30u8,
                218u8,
                34u8,
                94u8,
                70u8,
                121u8,
                26u8,
                169u8,
                41u8,
                139u8,
                1u8,
                96u8,
                100u8,
                143u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    previous: data.0,
                    current: data.1,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.previous),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.current),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for MinimumWeightUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&MinimumWeightUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &MinimumWeightUpdated) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `OperatorDeregistered(address,address)` and selector `0x31e0adfec71bccee37b6e83a90c2fedb17d8f1693fee863c4771e7bfe2aed580`.
```solidity
event OperatorDeregistered(address indexed operator, address indexed avs);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct OperatorDeregistered {
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub avs: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for OperatorDeregistered {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "OperatorDeregistered(address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                49u8,
                224u8,
                173u8,
                254u8,
                199u8,
                27u8,
                204u8,
                238u8,
                55u8,
                182u8,
                232u8,
                58u8,
                144u8,
                194u8,
                254u8,
                219u8,
                23u8,
                216u8,
                241u8,
                105u8,
                63u8,
                238u8,
                134u8,
                60u8,
                71u8,
                113u8,
                231u8,
                191u8,
                226u8,
                174u8,
                213u8,
                128u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    operator: topics.1,
                    avs: topics.2,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                ()
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.operator.clone(), self.avs.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.operator,
                );
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.avs,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for OperatorDeregistered {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OperatorDeregistered> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &OperatorDeregistered) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `OperatorRegistered(address,address)` and selector `0xa453db612af59e5521d6ab9284dc3e2d06af286eb1b1b7b771fce4716c19f2c1`.
```solidity
event OperatorRegistered(address indexed operator, address indexed avs);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct OperatorRegistered {
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub avs: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for OperatorRegistered {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "OperatorRegistered(address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                164u8,
                83u8,
                219u8,
                97u8,
                42u8,
                245u8,
                158u8,
                85u8,
                33u8,
                214u8,
                171u8,
                146u8,
                132u8,
                220u8,
                62u8,
                45u8,
                6u8,
                175u8,
                40u8,
                110u8,
                177u8,
                177u8,
                183u8,
                183u8,
                113u8,
                252u8,
                228u8,
                113u8,
                108u8,
                25u8,
                242u8,
                193u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    operator: topics.1,
                    avs: topics.2,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                ()
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.operator.clone(), self.avs.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.operator,
                );
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.avs,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for OperatorRegistered {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OperatorRegistered> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &OperatorRegistered) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `OperatorWeightUpdated(address,uint256,uint256)` and selector `0x88770dc862e47a7ed586907857eb1b75e4c5ffc8b707c7ee10eb74d6885fe594`.
```solidity
event OperatorWeightUpdated(address indexed operator, uint256 oldWeight, uint256 newWeight);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct OperatorWeightUpdated {
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub oldWeight: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub newWeight: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for OperatorWeightUpdated {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "OperatorWeightUpdated(address,uint256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                136u8,
                119u8,
                13u8,
                200u8,
                98u8,
                228u8,
                122u8,
                126u8,
                213u8,
                134u8,
                144u8,
                120u8,
                87u8,
                235u8,
                27u8,
                117u8,
                228u8,
                197u8,
                255u8,
                200u8,
                183u8,
                7u8,
                199u8,
                238u8,
                16u8,
                235u8,
                116u8,
                214u8,
                136u8,
                95u8,
                229u8,
                148u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    operator: topics.1,
                    oldWeight: data.0,
                    newWeight: data.1,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.oldWeight),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.newWeight),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.operator.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.operator,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for OperatorWeightUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OperatorWeightUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &OperatorWeightUpdated) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `OwnershipTransferred(address,address)` and selector `0x8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0`.
```solidity
event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct OwnershipTransferred {
        #[allow(missing_docs)]
        pub previousOwner: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub newOwner: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for OwnershipTransferred {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "OwnershipTransferred(address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                139u8,
                224u8,
                7u8,
                156u8,
                83u8,
                22u8,
                89u8,
                20u8,
                19u8,
                68u8,
                205u8,
                31u8,
                208u8,
                164u8,
                242u8,
                132u8,
                25u8,
                73u8,
                127u8,
                151u8,
                34u8,
                163u8,
                218u8,
                175u8,
                227u8,
                180u8,
                24u8,
                111u8,
                107u8,
                100u8,
                87u8,
                224u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    previousOwner: topics.1,
                    newOwner: topics.2,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                ()
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.previousOwner.clone(),
                    self.newOwner.clone(),
                )
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.previousOwner,
                );
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.newOwner,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for OwnershipTransferred {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OwnershipTransferred> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &OwnershipTransferred) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `QuorumUpdated(((address,uint96)[]),((address,uint96)[]))` and selector `0x23aad4e61744ece164130aa415c1616e80136b0f0770e56589438b90b269265e`.
```solidity
event QuorumUpdated(IECDSAStakeRegistryTypes.Quorum previous, IECDSAStakeRegistryTypes.Quorum current);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct QuorumUpdated {
        #[allow(missing_docs)]
        pub previous: <IECDSAStakeRegistryTypes::Quorum as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub current: <IECDSAStakeRegistryTypes::Quorum as alloy::sol_types::SolType>::RustType,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for QuorumUpdated {
            type DataTuple<'a> = (
                IECDSAStakeRegistryTypes::Quorum,
                IECDSAStakeRegistryTypes::Quorum,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "QuorumUpdated(((address,uint96)[]),((address,uint96)[]))";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                35u8,
                170u8,
                212u8,
                230u8,
                23u8,
                68u8,
                236u8,
                225u8,
                100u8,
                19u8,
                10u8,
                164u8,
                21u8,
                193u8,
                97u8,
                110u8,
                128u8,
                19u8,
                107u8,
                15u8,
                7u8,
                112u8,
                229u8,
                101u8,
                137u8,
                67u8,
                139u8,
                144u8,
                178u8,
                105u8,
                38u8,
                94u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    previous: data.0,
                    current: data.1,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <IECDSAStakeRegistryTypes::Quorum as alloy_sol_types::SolType>::tokenize(
                        &self.previous,
                    ),
                    <IECDSAStakeRegistryTypes::Quorum as alloy_sol_types::SolType>::tokenize(
                        &self.current,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for QuorumUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&QuorumUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &QuorumUpdated) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `SigningKeyUpdate(address,uint256,address,address)` and selector `0xd061168252f441733658f09e4d8f5b2d998ed4ef24a2bbfd6ceca52ea1315002`.
```solidity
event SigningKeyUpdate(address indexed operator, uint256 indexed updateBlock, address indexed newSigningKey, address oldSigningKey);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct SigningKeyUpdate {
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub updateBlock: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub newSigningKey: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub oldSigningKey: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for SigningKeyUpdate {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "SigningKeyUpdate(address,uint256,address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                208u8,
                97u8,
                22u8,
                130u8,
                82u8,
                244u8,
                65u8,
                115u8,
                54u8,
                88u8,
                240u8,
                158u8,
                77u8,
                143u8,
                91u8,
                45u8,
                153u8,
                142u8,
                212u8,
                239u8,
                36u8,
                162u8,
                187u8,
                253u8,
                108u8,
                236u8,
                165u8,
                46u8,
                161u8,
                49u8,
                80u8,
                2u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    operator: topics.1,
                    updateBlock: topics.2,
                    newSigningKey: topics.3,
                    oldSigningKey: data.0,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.oldSigningKey,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.operator.clone(),
                    self.updateBlock.clone(),
                    self.newSigningKey.clone(),
                )
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.operator,
                );
                out[2usize] = <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.updateBlock);
                out[3usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.newSigningKey,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for SigningKeyUpdate {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&SigningKeyUpdate> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &SigningKeyUpdate) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `ThresholdWeightUpdated(uint256)` and selector `0x9324f7e5a7c0288808a634ccde44b8e979676474b22e29ee9dd569b55e791a4b`.
```solidity
event ThresholdWeightUpdated(uint256 thresholdWeight);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct ThresholdWeightUpdated {
        #[allow(missing_docs)]
        pub thresholdWeight: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for ThresholdWeightUpdated {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "ThresholdWeightUpdated(uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                147u8,
                36u8,
                247u8,
                229u8,
                167u8,
                192u8,
                40u8,
                136u8,
                8u8,
                166u8,
                52u8,
                204u8,
                222u8,
                68u8,
                184u8,
                233u8,
                121u8,
                103u8,
                100u8,
                116u8,
                178u8,
                46u8,
                41u8,
                238u8,
                157u8,
                213u8,
                105u8,
                181u8,
                94u8,
                121u8,
                26u8,
                75u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { thresholdWeight: data.0 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.thresholdWeight),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for ThresholdWeightUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&ThresholdWeightUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &ThresholdWeightUpdated) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `TotalWeightUpdated(uint256,uint256)` and selector `0x86dcf86b12dfeedea74ae9300dbdaa193bcce5809369c8177ea2f4eaaa65729b`.
```solidity
event TotalWeightUpdated(uint256 oldTotalWeight, uint256 newTotalWeight);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct TotalWeightUpdated {
        #[allow(missing_docs)]
        pub oldTotalWeight: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub newTotalWeight: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for TotalWeightUpdated {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "TotalWeightUpdated(uint256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                134u8,
                220u8,
                248u8,
                107u8,
                18u8,
                223u8,
                238u8,
                222u8,
                167u8,
                74u8,
                233u8,
                48u8,
                13u8,
                189u8,
                170u8,
                25u8,
                59u8,
                204u8,
                229u8,
                128u8,
                147u8,
                105u8,
                200u8,
                23u8,
                126u8,
                162u8,
                244u8,
                234u8,
                170u8,
                101u8,
                114u8,
                155u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    oldTotalWeight: data.0,
                    newTotalWeight: data.1,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.oldTotalWeight),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.newTotalWeight),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for TotalWeightUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&TotalWeightUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &TotalWeightUpdated) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `UpdateMinimumWeight(uint256,uint256)` and selector `0x1ea42186b305fa37310450d9fb87ea1e8f0c7f447e771479e3b27634bfe84dc1`.
```solidity
event UpdateMinimumWeight(uint256 oldMinimumWeight, uint256 newMinimumWeight);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct UpdateMinimumWeight {
        #[allow(missing_docs)]
        pub oldMinimumWeight: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub newMinimumWeight: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for UpdateMinimumWeight {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "UpdateMinimumWeight(uint256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                30u8,
                164u8,
                33u8,
                134u8,
                179u8,
                5u8,
                250u8,
                55u8,
                49u8,
                4u8,
                80u8,
                217u8,
                251u8,
                135u8,
                234u8,
                30u8,
                143u8,
                12u8,
                127u8,
                68u8,
                126u8,
                119u8,
                20u8,
                121u8,
                227u8,
                178u8,
                118u8,
                52u8,
                191u8,
                232u8,
                77u8,
                193u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    oldMinimumWeight: data.0,
                    newMinimumWeight: data.1,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.oldMinimumWeight),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.newMinimumWeight),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for UpdateMinimumWeight {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&UpdateMinimumWeight> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &UpdateMinimumWeight) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Constructor`.
```solidity
constructor(address _delegationManager);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct constructorCall {
        #[allow(missing_docs)]
        pub _delegationManager: alloy::sol_types::private::Address,
    }
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<constructorCall> for UnderlyingRustTuple<'_> {
                fn from(value: constructorCall) -> Self {
                    (value._delegationManager,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for constructorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _delegationManager: tuple.0,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolConstructor for constructorCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._delegationManager,
                    ),
                )
            }
        }
    };
    /**Function with signature `deregisterOperator()` and selector `0x857dc190`.
```solidity
function deregisterOperator() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct deregisterOperatorCall {}
    ///Container type for the return parameters of the [`deregisterOperator()`](deregisterOperatorCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct deregisterOperatorReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<deregisterOperatorCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: deregisterOperatorCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for deregisterOperatorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<deregisterOperatorReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: deregisterOperatorReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for deregisterOperatorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for deregisterOperatorCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = deregisterOperatorReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "deregisterOperator()";
            const SELECTOR: [u8; 4] = [133u8, 125u8, 193u8, 144u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `getLastCheckpointOperatorWeight(address)` and selector `0x3b242e4a`.
```solidity
function getLastCheckpointOperatorWeight(address operator) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getLastCheckpointOperatorWeightCall {
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`getLastCheckpointOperatorWeight(address)`](getLastCheckpointOperatorWeightCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getLastCheckpointOperatorWeightReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getLastCheckpointOperatorWeightCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getLastCheckpointOperatorWeightCall) -> Self {
                    (value.operator,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getLastCheckpointOperatorWeightCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { operator: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getLastCheckpointOperatorWeightReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getLastCheckpointOperatorWeightReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getLastCheckpointOperatorWeightReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getLastCheckpointOperatorWeightCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getLastCheckpointOperatorWeightReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getLastCheckpointOperatorWeight(address)";
            const SELECTOR: [u8; 4] = [59u8, 36u8, 46u8, 74u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.operator,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `getLastCheckpointThresholdWeight()` and selector `0xb933fa74`.
```solidity
function getLastCheckpointThresholdWeight() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getLastCheckpointThresholdWeightCall {}
    ///Container type for the return parameters of the [`getLastCheckpointThresholdWeight()`](getLastCheckpointThresholdWeightCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getLastCheckpointThresholdWeightReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getLastCheckpointThresholdWeightCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getLastCheckpointThresholdWeightCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getLastCheckpointThresholdWeightCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getLastCheckpointThresholdWeightReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getLastCheckpointThresholdWeightReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getLastCheckpointThresholdWeightReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getLastCheckpointThresholdWeightCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getLastCheckpointThresholdWeightReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getLastCheckpointThresholdWeight()";
            const SELECTOR: [u8; 4] = [185u8, 51u8, 250u8, 116u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `getLastCheckpointThresholdWeightAtBlock(uint32)` and selector `0x1e4cd85e`.
```solidity
function getLastCheckpointThresholdWeightAtBlock(uint32 blockNumber) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getLastCheckpointThresholdWeightAtBlockCall {
        #[allow(missing_docs)]
        pub blockNumber: u32,
    }
    ///Container type for the return parameters of the [`getLastCheckpointThresholdWeightAtBlock(uint32)`](getLastCheckpointThresholdWeightAtBlockCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getLastCheckpointThresholdWeightAtBlockReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u32,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getLastCheckpointThresholdWeightAtBlockCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getLastCheckpointThresholdWeightAtBlockCall) -> Self {
                    (value.blockNumber,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getLastCheckpointThresholdWeightAtBlockCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { blockNumber: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getLastCheckpointThresholdWeightAtBlockReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getLastCheckpointThresholdWeightAtBlockReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getLastCheckpointThresholdWeightAtBlockReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getLastCheckpointThresholdWeightAtBlockCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getLastCheckpointThresholdWeightAtBlockReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getLastCheckpointThresholdWeightAtBlock(uint32)";
            const SELECTOR: [u8; 4] = [30u8, 76u8, 216u8, 94u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.blockNumber),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `getLastCheckpointTotalWeight()` and selector `0x314f3a49`.
```solidity
function getLastCheckpointTotalWeight() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getLastCheckpointTotalWeightCall {}
    ///Container type for the return parameters of the [`getLastCheckpointTotalWeight()`](getLastCheckpointTotalWeightCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getLastCheckpointTotalWeightReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getLastCheckpointTotalWeightCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getLastCheckpointTotalWeightCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getLastCheckpointTotalWeightCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getLastCheckpointTotalWeightReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getLastCheckpointTotalWeightReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getLastCheckpointTotalWeightReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getLastCheckpointTotalWeightCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getLastCheckpointTotalWeightReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getLastCheckpointTotalWeight()";
            const SELECTOR: [u8; 4] = [49u8, 79u8, 58u8, 73u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `getLastCheckpointTotalWeightAtBlock(uint32)` and selector `0x0dba3394`.
```solidity
function getLastCheckpointTotalWeightAtBlock(uint32 blockNumber) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getLastCheckpointTotalWeightAtBlockCall {
        #[allow(missing_docs)]
        pub blockNumber: u32,
    }
    ///Container type for the return parameters of the [`getLastCheckpointTotalWeightAtBlock(uint32)`](getLastCheckpointTotalWeightAtBlockCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getLastCheckpointTotalWeightAtBlockReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u32,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getLastCheckpointTotalWeightAtBlockCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getLastCheckpointTotalWeightAtBlockCall) -> Self {
                    (value.blockNumber,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getLastCheckpointTotalWeightAtBlockCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { blockNumber: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getLastCheckpointTotalWeightAtBlockReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getLastCheckpointTotalWeightAtBlockReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getLastCheckpointTotalWeightAtBlockReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getLastCheckpointTotalWeightAtBlockCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getLastCheckpointTotalWeightAtBlockReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getLastCheckpointTotalWeightAtBlock(uint32)";
            const SELECTOR: [u8; 4] = [13u8, 186u8, 51u8, 148u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.blockNumber),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `getLatestOperatorSigningKey(address)` and selector `0xa2ce5fd1`.
```solidity
function getLatestOperatorSigningKey(address operator) external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getLatestOperatorSigningKeyCall {
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`getLatestOperatorSigningKey(address)`](getLatestOperatorSigningKeyCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getLatestOperatorSigningKeyReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getLatestOperatorSigningKeyCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getLatestOperatorSigningKeyCall) -> Self {
                    (value.operator,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getLatestOperatorSigningKeyCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { operator: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getLatestOperatorSigningKeyReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getLatestOperatorSigningKeyReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getLatestOperatorSigningKeyReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getLatestOperatorSigningKeyCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getLatestOperatorSigningKeyReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getLatestOperatorSigningKey(address)";
            const SELECTOR: [u8; 4] = [162u8, 206u8, 95u8, 209u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.operator,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `getOperatorSigningKeyAtBlock(address,uint256)` and selector `0x5e1042e8`.
```solidity
function getOperatorSigningKeyAtBlock(address operator, uint256 blockNumber) external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorSigningKeyAtBlockCall {
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub blockNumber: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`getOperatorSigningKeyAtBlock(address,uint256)`](getOperatorSigningKeyAtBlockCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorSigningKeyAtBlockReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getOperatorSigningKeyAtBlockCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorSigningKeyAtBlockCall) -> Self {
                    (value.operator, value.blockNumber)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getOperatorSigningKeyAtBlockCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operator: tuple.0,
                        blockNumber: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getOperatorSigningKeyAtBlockReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorSigningKeyAtBlockReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getOperatorSigningKeyAtBlockReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getOperatorSigningKeyAtBlockCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getOperatorSigningKeyAtBlockReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getOperatorSigningKeyAtBlock(address,uint256)";
            const SELECTOR: [u8; 4] = [94u8, 16u8, 66u8, 232u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.operator,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.blockNumber),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `getOperatorWeight(address)` and selector `0x98ec1ac9`.
```solidity
function getOperatorWeight(address operator) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorWeightCall {
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`getOperatorWeight(address)`](getOperatorWeightCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorWeightReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getOperatorWeightCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorWeightCall) -> Self {
                    (value.operator,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getOperatorWeightCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { operator: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getOperatorWeightReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorWeightReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getOperatorWeightReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getOperatorWeightCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getOperatorWeightReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getOperatorWeight(address)";
            const SELECTOR: [u8; 4] = [152u8, 236u8, 26u8, 201u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.operator,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `getOperatorWeightAtBlock(address,uint32)` and selector `0x955f2d90`.
```solidity
function getOperatorWeightAtBlock(address operator, uint32 blockNumber) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorWeightAtBlockCall {
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub blockNumber: u32,
    }
    ///Container type for the return parameters of the [`getOperatorWeightAtBlock(address,uint32)`](getOperatorWeightAtBlockCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorWeightAtBlockReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address, u32);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getOperatorWeightAtBlockCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorWeightAtBlockCall) -> Self {
                    (value.operator, value.blockNumber)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getOperatorWeightAtBlockCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operator: tuple.0,
                        blockNumber: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getOperatorWeightAtBlockReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorWeightAtBlockReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getOperatorWeightAtBlockReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getOperatorWeightAtBlockCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<32>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getOperatorWeightAtBlockReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getOperatorWeightAtBlock(address,uint32)";
            const SELECTOR: [u8; 4] = [149u8, 95u8, 45u8, 144u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.operator,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.blockNumber),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `initialize(address,uint256,((address,uint96)[]))` and selector `0xab118995`.
```solidity
function initialize(address _serviceManager, uint256 thresholdWeight, IECDSAStakeRegistryTypes.Quorum memory quorum) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initializeCall {
        #[allow(missing_docs)]
        pub _serviceManager: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub thresholdWeight: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub quorum: <IECDSAStakeRegistryTypes::Quorum as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`initialize(address,uint256,((address,uint96)[]))`](initializeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initializeReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                IECDSAStakeRegistryTypes::Quorum,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
                <IECDSAStakeRegistryTypes::Quorum as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<initializeCall> for UnderlyingRustTuple<'_> {
                fn from(value: initializeCall) -> Self {
                    (value._serviceManager, value.thresholdWeight, value.quorum)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for initializeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _serviceManager: tuple.0,
                        thresholdWeight: tuple.1,
                        quorum: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<initializeReturn> for UnderlyingRustTuple<'_> {
                fn from(value: initializeReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for initializeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for initializeCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                IECDSAStakeRegistryTypes::Quorum,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = initializeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "initialize(address,uint256,((address,uint96)[]))";
            const SELECTOR: [u8; 4] = [171u8, 17u8, 137u8, 149u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._serviceManager,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.thresholdWeight),
                    <IECDSAStakeRegistryTypes::Quorum as alloy_sol_types::SolType>::tokenize(
                        &self.quorum,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `isValidSignature(bytes32,bytes)` and selector `0x1626ba7e`.
```solidity
function isValidSignature(bytes32 digest, bytes memory _signatureData) external view returns (bytes4);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isValidSignatureCall {
        #[allow(missing_docs)]
        pub digest: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub _signatureData: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`isValidSignature(bytes32,bytes)`](isValidSignatureCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isValidSignatureReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::FixedBytes<4>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
                alloy::sol_types::private::Bytes,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<isValidSignatureCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: isValidSignatureCall) -> Self {
                    (value.digest, value._signatureData)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isValidSignatureCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        digest: tuple.0,
                        _signatureData: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<4>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<4>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<isValidSignatureReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: isValidSignatureReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isValidSignatureReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isValidSignatureCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = isValidSignatureReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<4>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "isValidSignature(bytes32,bytes)";
            const SELECTOR: [u8; 4] = [22u8, 38u8, 186u8, 126u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.digest),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self._signatureData,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `minimumWeight()` and selector `0x40bf2fb7`.
```solidity
function minimumWeight() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct minimumWeightCall {}
    ///Container type for the return parameters of the [`minimumWeight()`](minimumWeightCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct minimumWeightReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<minimumWeightCall> for UnderlyingRustTuple<'_> {
                fn from(value: minimumWeightCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for minimumWeightCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<minimumWeightReturn> for UnderlyingRustTuple<'_> {
                fn from(value: minimumWeightReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for minimumWeightReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for minimumWeightCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = minimumWeightReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "minimumWeight()";
            const SELECTOR: [u8; 4] = [64u8, 191u8, 47u8, 183u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `operatorRegistered(address)` and selector `0xec7fbb31`.
```solidity
function operatorRegistered(address operator) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct operatorRegisteredCall {
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`operatorRegistered(address)`](operatorRegisteredCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct operatorRegisteredReturn {
        #[allow(missing_docs)]
        pub _0: bool,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<operatorRegisteredCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: operatorRegisteredCall) -> Self {
                    (value.operator,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for operatorRegisteredCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { operator: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (bool,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<operatorRegisteredReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: operatorRegisteredReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for operatorRegisteredReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for operatorRegisteredCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = operatorRegisteredReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "operatorRegistered(address)";
            const SELECTOR: [u8; 4] = [236u8, 127u8, 187u8, 49u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.operator,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `owner()` and selector `0x8da5cb5b`.
```solidity
function owner() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ownerCall {}
    ///Container type for the return parameters of the [`owner()`](ownerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ownerReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<ownerCall> for UnderlyingRustTuple<'_> {
                fn from(value: ownerCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for ownerCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<ownerReturn> for UnderlyingRustTuple<'_> {
                fn from(value: ownerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for ownerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for ownerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = ownerReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "owner()";
            const SELECTOR: [u8; 4] = [141u8, 165u8, 203u8, 91u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `quorum()` and selector `0x1703a018`.
```solidity
function quorum() external view returns (IECDSAStakeRegistryTypes.Quorum memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct quorumCall {}
    ///Container type for the return parameters of the [`quorum()`](quorumCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct quorumReturn {
        #[allow(missing_docs)]
        pub _0: <IECDSAStakeRegistryTypes::Quorum as alloy::sol_types::SolType>::RustType,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<quorumCall> for UnderlyingRustTuple<'_> {
                fn from(value: quorumCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for quorumCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (IECDSAStakeRegistryTypes::Quorum,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IECDSAStakeRegistryTypes::Quorum as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<quorumReturn> for UnderlyingRustTuple<'_> {
                fn from(value: quorumReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for quorumReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for quorumCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = quorumReturn;
            type ReturnTuple<'a> = (IECDSAStakeRegistryTypes::Quorum,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "quorum()";
            const SELECTOR: [u8; 4] = [23u8, 3u8, 160u8, 24u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `registerOperatorWithSignature((bytes,bytes32,uint256),address)` and selector `0x3d5611f6`.
```solidity
function registerOperatorWithSignature(ISignatureUtils.SignatureWithSaltAndExpiry memory operatorSignature, address signingKey) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct registerOperatorWithSignatureCall {
        #[allow(missing_docs)]
        pub operatorSignature: <ISignatureUtils::SignatureWithSaltAndExpiry as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub signingKey: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`registerOperatorWithSignature((bytes,bytes32,uint256),address)`](registerOperatorWithSignatureCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct registerOperatorWithSignatureReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                ISignatureUtils::SignatureWithSaltAndExpiry,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <ISignatureUtils::SignatureWithSaltAndExpiry as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::Address,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<registerOperatorWithSignatureCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: registerOperatorWithSignatureCall) -> Self {
                    (value.operatorSignature, value.signingKey)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for registerOperatorWithSignatureCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operatorSignature: tuple.0,
                        signingKey: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<registerOperatorWithSignatureReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: registerOperatorWithSignatureReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for registerOperatorWithSignatureReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for registerOperatorWithSignatureCall {
            type Parameters<'a> = (
                ISignatureUtils::SignatureWithSaltAndExpiry,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = registerOperatorWithSignatureReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "registerOperatorWithSignature((bytes,bytes32,uint256),address)";
            const SELECTOR: [u8; 4] = [61u8, 86u8, 17u8, 246u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <ISignatureUtils::SignatureWithSaltAndExpiry as alloy_sol_types::SolType>::tokenize(
                        &self.operatorSignature,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.signingKey,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `renounceOwnership()` and selector `0x715018a6`.
```solidity
function renounceOwnership() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct renounceOwnershipCall {}
    ///Container type for the return parameters of the [`renounceOwnership()`](renounceOwnershipCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct renounceOwnershipReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<renounceOwnershipCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: renounceOwnershipCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for renounceOwnershipCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<renounceOwnershipReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: renounceOwnershipReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for renounceOwnershipReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for renounceOwnershipCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = renounceOwnershipReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "renounceOwnership()";
            const SELECTOR: [u8; 4] = [113u8, 80u8, 24u8, 166u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `transferOwnership(address)` and selector `0xf2fde38b`.
```solidity
function transferOwnership(address newOwner) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct transferOwnershipCall {
        #[allow(missing_docs)]
        pub newOwner: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`transferOwnership(address)`](transferOwnershipCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct transferOwnershipReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<transferOwnershipCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: transferOwnershipCall) -> Self {
                    (value.newOwner,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for transferOwnershipCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { newOwner: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<transferOwnershipReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: transferOwnershipReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for transferOwnershipReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for transferOwnershipCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = transferOwnershipReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "transferOwnership(address)";
            const SELECTOR: [u8; 4] = [242u8, 253u8, 227u8, 139u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.newOwner,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `updateMinimumWeight(uint256,address[])` and selector `0x696255be`.
```solidity
function updateMinimumWeight(uint256 newMinimumWeight, address[] memory operators) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateMinimumWeightCall {
        #[allow(missing_docs)]
        pub newMinimumWeight: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub operators: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
    }
    ///Container type for the return parameters of the [`updateMinimumWeight(uint256,address[])`](updateMinimumWeightCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateMinimumWeightReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<updateMinimumWeightCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: updateMinimumWeightCall) -> Self {
                    (value.newMinimumWeight, value.operators)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for updateMinimumWeightCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        newMinimumWeight: tuple.0,
                        operators: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<updateMinimumWeightReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: updateMinimumWeightReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for updateMinimumWeightReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for updateMinimumWeightCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = updateMinimumWeightReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "updateMinimumWeight(uint256,address[])";
            const SELECTOR: [u8; 4] = [105u8, 98u8, 85u8, 190u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.newMinimumWeight),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.operators),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `updateOperatorSigningKey(address)` and selector `0x743c31f4`.
```solidity
function updateOperatorSigningKey(address newSigningKey) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateOperatorSigningKeyCall {
        #[allow(missing_docs)]
        pub newSigningKey: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`updateOperatorSigningKey(address)`](updateOperatorSigningKeyCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateOperatorSigningKeyReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<updateOperatorSigningKeyCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: updateOperatorSigningKeyCall) -> Self {
                    (value.newSigningKey,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for updateOperatorSigningKeyCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { newSigningKey: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<updateOperatorSigningKeyReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: updateOperatorSigningKeyReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for updateOperatorSigningKeyReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for updateOperatorSigningKeyCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = updateOperatorSigningKeyReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "updateOperatorSigningKey(address)";
            const SELECTOR: [u8; 4] = [116u8, 60u8, 49u8, 244u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.newSigningKey,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `updateOperators(address[])` and selector `0x00cf2ab5`.
```solidity
function updateOperators(address[] memory operators) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateOperatorsCall {
        #[allow(missing_docs)]
        pub operators: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
    }
    ///Container type for the return parameters of the [`updateOperators(address[])`](updateOperatorsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateOperatorsReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<updateOperatorsCall> for UnderlyingRustTuple<'_> {
                fn from(value: updateOperatorsCall) -> Self {
                    (value.operators,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for updateOperatorsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { operators: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<updateOperatorsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: updateOperatorsReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for updateOperatorsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for updateOperatorsCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = updateOperatorsReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "updateOperators(address[])";
            const SELECTOR: [u8; 4] = [0u8, 207u8, 42u8, 181u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.operators),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `updateOperatorsForQuorum(address[][],bytes)` and selector `0x5140a548`.
```solidity
function updateOperatorsForQuorum(address[][] memory operatorsPerQuorum, bytes memory) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateOperatorsForQuorumCall {
        #[allow(missing_docs)]
        pub operatorsPerQuorum: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
        >,
        #[allow(missing_docs)]
        pub _1: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`updateOperatorsForQuorum(address[][],bytes)`](updateOperatorsForQuorumCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateOperatorsForQuorumReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    >,
                >,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
                >,
                alloy::sol_types::private::Bytes,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<updateOperatorsForQuorumCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: updateOperatorsForQuorumCall) -> Self {
                    (value.operatorsPerQuorum, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for updateOperatorsForQuorumCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operatorsPerQuorum: tuple.0,
                        _1: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<updateOperatorsForQuorumReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: updateOperatorsForQuorumReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for updateOperatorsForQuorumReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for updateOperatorsForQuorumCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    >,
                >,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = updateOperatorsForQuorumReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "updateOperatorsForQuorum(address[][],bytes)";
            const SELECTOR: [u8; 4] = [81u8, 64u8, 165u8, 72u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Array<
                            alloy::sol_types::sol_data::Address,
                        >,
                    > as alloy_sol_types::SolType>::tokenize(&self.operatorsPerQuorum),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self._1,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `updateQuorumConfig(((address,uint96)[]),address[])` and selector `0xdec5d1f6`.
```solidity
function updateQuorumConfig(IECDSAStakeRegistryTypes.Quorum memory quorum, address[] memory operators) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateQuorumConfigCall {
        #[allow(missing_docs)]
        pub quorum: <IECDSAStakeRegistryTypes::Quorum as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub operators: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
    }
    ///Container type for the return parameters of the [`updateQuorumConfig(((address,uint96)[]),address[])`](updateQuorumConfigCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateQuorumConfigReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                IECDSAStakeRegistryTypes::Quorum,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IECDSAStakeRegistryTypes::Quorum as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<updateQuorumConfigCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: updateQuorumConfigCall) -> Self {
                    (value.quorum, value.operators)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for updateQuorumConfigCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        quorum: tuple.0,
                        operators: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<updateQuorumConfigReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: updateQuorumConfigReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for updateQuorumConfigReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for updateQuorumConfigCall {
            type Parameters<'a> = (
                IECDSAStakeRegistryTypes::Quorum,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = updateQuorumConfigReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "updateQuorumConfig(((address,uint96)[]),address[])";
            const SELECTOR: [u8; 4] = [222u8, 197u8, 209u8, 246u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <IECDSAStakeRegistryTypes::Quorum as alloy_sol_types::SolType>::tokenize(
                        &self.quorum,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.operators),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `updateStakeThreshold(uint256)` and selector `0x5ef53329`.
```solidity
function updateStakeThreshold(uint256 thresholdWeight) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateStakeThresholdCall {
        #[allow(missing_docs)]
        pub thresholdWeight: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`updateStakeThreshold(uint256)`](updateStakeThresholdCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateStakeThresholdReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<updateStakeThresholdCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: updateStakeThresholdCall) -> Self {
                    (value.thresholdWeight,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for updateStakeThresholdCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { thresholdWeight: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<updateStakeThresholdReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: updateStakeThresholdReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for updateStakeThresholdReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for updateStakeThresholdCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = updateStakeThresholdReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "updateStakeThreshold(uint256)";
            const SELECTOR: [u8; 4] = [94u8, 245u8, 51u8, 41u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.thresholdWeight),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    ///Container for all the [`ECDSAStakeRegistry`](self) function calls.
    pub enum ECDSAStakeRegistryCalls {
        #[allow(missing_docs)]
        deregisterOperator(deregisterOperatorCall),
        #[allow(missing_docs)]
        getLastCheckpointOperatorWeight(getLastCheckpointOperatorWeightCall),
        #[allow(missing_docs)]
        getLastCheckpointThresholdWeight(getLastCheckpointThresholdWeightCall),
        #[allow(missing_docs)]
        getLastCheckpointThresholdWeightAtBlock(
            getLastCheckpointThresholdWeightAtBlockCall,
        ),
        #[allow(missing_docs)]
        getLastCheckpointTotalWeight(getLastCheckpointTotalWeightCall),
        #[allow(missing_docs)]
        getLastCheckpointTotalWeightAtBlock(getLastCheckpointTotalWeightAtBlockCall),
        #[allow(missing_docs)]
        getLatestOperatorSigningKey(getLatestOperatorSigningKeyCall),
        #[allow(missing_docs)]
        getOperatorSigningKeyAtBlock(getOperatorSigningKeyAtBlockCall),
        #[allow(missing_docs)]
        getOperatorWeight(getOperatorWeightCall),
        #[allow(missing_docs)]
        getOperatorWeightAtBlock(getOperatorWeightAtBlockCall),
        #[allow(missing_docs)]
        initialize(initializeCall),
        #[allow(missing_docs)]
        isValidSignature(isValidSignatureCall),
        #[allow(missing_docs)]
        minimumWeight(minimumWeightCall),
        #[allow(missing_docs)]
        operatorRegistered(operatorRegisteredCall),
        #[allow(missing_docs)]
        owner(ownerCall),
        #[allow(missing_docs)]
        quorum(quorumCall),
        #[allow(missing_docs)]
        registerOperatorWithSignature(registerOperatorWithSignatureCall),
        #[allow(missing_docs)]
        renounceOwnership(renounceOwnershipCall),
        #[allow(missing_docs)]
        transferOwnership(transferOwnershipCall),
        #[allow(missing_docs)]
        updateMinimumWeight(updateMinimumWeightCall),
        #[allow(missing_docs)]
        updateOperatorSigningKey(updateOperatorSigningKeyCall),
        #[allow(missing_docs)]
        updateOperators(updateOperatorsCall),
        #[allow(missing_docs)]
        updateOperatorsForQuorum(updateOperatorsForQuorumCall),
        #[allow(missing_docs)]
        updateQuorumConfig(updateQuorumConfigCall),
        #[allow(missing_docs)]
        updateStakeThreshold(updateStakeThresholdCall),
    }
    #[automatically_derived]
    impl ECDSAStakeRegistryCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [0u8, 207u8, 42u8, 181u8],
            [13u8, 186u8, 51u8, 148u8],
            [22u8, 38u8, 186u8, 126u8],
            [23u8, 3u8, 160u8, 24u8],
            [30u8, 76u8, 216u8, 94u8],
            [49u8, 79u8, 58u8, 73u8],
            [59u8, 36u8, 46u8, 74u8],
            [61u8, 86u8, 17u8, 246u8],
            [64u8, 191u8, 47u8, 183u8],
            [81u8, 64u8, 165u8, 72u8],
            [94u8, 16u8, 66u8, 232u8],
            [94u8, 245u8, 51u8, 41u8],
            [105u8, 98u8, 85u8, 190u8],
            [113u8, 80u8, 24u8, 166u8],
            [116u8, 60u8, 49u8, 244u8],
            [133u8, 125u8, 193u8, 144u8],
            [141u8, 165u8, 203u8, 91u8],
            [149u8, 95u8, 45u8, 144u8],
            [152u8, 236u8, 26u8, 201u8],
            [162u8, 206u8, 95u8, 209u8],
            [171u8, 17u8, 137u8, 149u8],
            [185u8, 51u8, 250u8, 116u8],
            [222u8, 197u8, 209u8, 246u8],
            [236u8, 127u8, 187u8, 49u8],
            [242u8, 253u8, 227u8, 139u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for ECDSAStakeRegistryCalls {
        const NAME: &'static str = "ECDSAStakeRegistryCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 25usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::deregisterOperator(_) => {
                    <deregisterOperatorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getLastCheckpointOperatorWeight(_) => {
                    <getLastCheckpointOperatorWeightCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getLastCheckpointThresholdWeight(_) => {
                    <getLastCheckpointThresholdWeightCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getLastCheckpointThresholdWeightAtBlock(_) => {
                    <getLastCheckpointThresholdWeightAtBlockCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getLastCheckpointTotalWeight(_) => {
                    <getLastCheckpointTotalWeightCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getLastCheckpointTotalWeightAtBlock(_) => {
                    <getLastCheckpointTotalWeightAtBlockCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getLatestOperatorSigningKey(_) => {
                    <getLatestOperatorSigningKeyCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getOperatorSigningKeyAtBlock(_) => {
                    <getOperatorSigningKeyAtBlockCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getOperatorWeight(_) => {
                    <getOperatorWeightCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getOperatorWeightAtBlock(_) => {
                    <getOperatorWeightAtBlockCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::initialize(_) => {
                    <initializeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::isValidSignature(_) => {
                    <isValidSignatureCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::minimumWeight(_) => {
                    <minimumWeightCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::operatorRegistered(_) => {
                    <operatorRegisteredCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::owner(_) => <ownerCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::quorum(_) => <quorumCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::registerOperatorWithSignature(_) => {
                    <registerOperatorWithSignatureCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::renounceOwnership(_) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::transferOwnership(_) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::updateMinimumWeight(_) => {
                    <updateMinimumWeightCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::updateOperatorSigningKey(_) => {
                    <updateOperatorSigningKeyCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::updateOperators(_) => {
                    <updateOperatorsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::updateOperatorsForQuorum(_) => {
                    <updateOperatorsForQuorumCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::updateQuorumConfig(_) => {
                    <updateQuorumConfigCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::updateStakeThreshold(_) => {
                    <updateStakeThresholdCall as alloy_sol_types::SolCall>::SELECTOR
                }
            }
        }
        #[inline]
        fn selector_at(i: usize) -> ::core::option::Option<[u8; 4]> {
            Self::SELECTORS.get(i).copied()
        }
        #[inline]
        fn valid_selector(selector: [u8; 4]) -> bool {
            Self::SELECTORS.binary_search(&selector).is_ok()
        }
        #[inline]
        #[allow(non_snake_case)]
        fn abi_decode_raw(
            selector: [u8; 4],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_SHIMS: &[fn(
                &[u8],
                bool,
            ) -> alloy_sol_types::Result<ECDSAStakeRegistryCalls>] = &[
                {
                    fn updateOperators(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryCalls> {
                        <updateOperatorsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ECDSAStakeRegistryCalls::updateOperators)
                    }
                    updateOperators
                },
                {
                    fn getLastCheckpointTotalWeightAtBlock(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryCalls> {
                        <getLastCheckpointTotalWeightAtBlockCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                ECDSAStakeRegistryCalls::getLastCheckpointTotalWeightAtBlock,
                            )
                    }
                    getLastCheckpointTotalWeightAtBlock
                },
                {
                    fn isValidSignature(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryCalls> {
                        <isValidSignatureCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ECDSAStakeRegistryCalls::isValidSignature)
                    }
                    isValidSignature
                },
                {
                    fn quorum(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryCalls> {
                        <quorumCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ECDSAStakeRegistryCalls::quorum)
                    }
                    quorum
                },
                {
                    fn getLastCheckpointThresholdWeightAtBlock(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryCalls> {
                        <getLastCheckpointThresholdWeightAtBlockCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                ECDSAStakeRegistryCalls::getLastCheckpointThresholdWeightAtBlock,
                            )
                    }
                    getLastCheckpointThresholdWeightAtBlock
                },
                {
                    fn getLastCheckpointTotalWeight(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryCalls> {
                        <getLastCheckpointTotalWeightCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ECDSAStakeRegistryCalls::getLastCheckpointTotalWeight)
                    }
                    getLastCheckpointTotalWeight
                },
                {
                    fn getLastCheckpointOperatorWeight(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryCalls> {
                        <getLastCheckpointOperatorWeightCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                ECDSAStakeRegistryCalls::getLastCheckpointOperatorWeight,
                            )
                    }
                    getLastCheckpointOperatorWeight
                },
                {
                    fn registerOperatorWithSignature(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryCalls> {
                        <registerOperatorWithSignatureCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ECDSAStakeRegistryCalls::registerOperatorWithSignature)
                    }
                    registerOperatorWithSignature
                },
                {
                    fn minimumWeight(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryCalls> {
                        <minimumWeightCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ECDSAStakeRegistryCalls::minimumWeight)
                    }
                    minimumWeight
                },
                {
                    fn updateOperatorsForQuorum(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryCalls> {
                        <updateOperatorsForQuorumCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ECDSAStakeRegistryCalls::updateOperatorsForQuorum)
                    }
                    updateOperatorsForQuorum
                },
                {
                    fn getOperatorSigningKeyAtBlock(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryCalls> {
                        <getOperatorSigningKeyAtBlockCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ECDSAStakeRegistryCalls::getOperatorSigningKeyAtBlock)
                    }
                    getOperatorSigningKeyAtBlock
                },
                {
                    fn updateStakeThreshold(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryCalls> {
                        <updateStakeThresholdCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ECDSAStakeRegistryCalls::updateStakeThreshold)
                    }
                    updateStakeThreshold
                },
                {
                    fn updateMinimumWeight(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryCalls> {
                        <updateMinimumWeightCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ECDSAStakeRegistryCalls::updateMinimumWeight)
                    }
                    updateMinimumWeight
                },
                {
                    fn renounceOwnership(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryCalls> {
                        <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ECDSAStakeRegistryCalls::renounceOwnership)
                    }
                    renounceOwnership
                },
                {
                    fn updateOperatorSigningKey(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryCalls> {
                        <updateOperatorSigningKeyCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ECDSAStakeRegistryCalls::updateOperatorSigningKey)
                    }
                    updateOperatorSigningKey
                },
                {
                    fn deregisterOperator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryCalls> {
                        <deregisterOperatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ECDSAStakeRegistryCalls::deregisterOperator)
                    }
                    deregisterOperator
                },
                {
                    fn owner(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryCalls> {
                        <ownerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ECDSAStakeRegistryCalls::owner)
                    }
                    owner
                },
                {
                    fn getOperatorWeightAtBlock(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryCalls> {
                        <getOperatorWeightAtBlockCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ECDSAStakeRegistryCalls::getOperatorWeightAtBlock)
                    }
                    getOperatorWeightAtBlock
                },
                {
                    fn getOperatorWeight(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryCalls> {
                        <getOperatorWeightCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ECDSAStakeRegistryCalls::getOperatorWeight)
                    }
                    getOperatorWeight
                },
                {
                    fn getLatestOperatorSigningKey(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryCalls> {
                        <getLatestOperatorSigningKeyCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ECDSAStakeRegistryCalls::getLatestOperatorSigningKey)
                    }
                    getLatestOperatorSigningKey
                },
                {
                    fn initialize(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryCalls> {
                        <initializeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ECDSAStakeRegistryCalls::initialize)
                    }
                    initialize
                },
                {
                    fn getLastCheckpointThresholdWeight(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryCalls> {
                        <getLastCheckpointThresholdWeightCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                ECDSAStakeRegistryCalls::getLastCheckpointThresholdWeight,
                            )
                    }
                    getLastCheckpointThresholdWeight
                },
                {
                    fn updateQuorumConfig(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryCalls> {
                        <updateQuorumConfigCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ECDSAStakeRegistryCalls::updateQuorumConfig)
                    }
                    updateQuorumConfig
                },
                {
                    fn operatorRegistered(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryCalls> {
                        <operatorRegisteredCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ECDSAStakeRegistryCalls::operatorRegistered)
                    }
                    operatorRegistered
                },
                {
                    fn transferOwnership(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryCalls> {
                        <transferOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ECDSAStakeRegistryCalls::transferOwnership)
                    }
                    transferOwnership
                },
            ];
            let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                return Err(
                    alloy_sol_types::Error::unknown_selector(
                        <Self as alloy_sol_types::SolInterface>::NAME,
                        selector,
                    ),
                );
            };
            DECODE_SHIMS[idx](data, validate)
        }
        #[inline]
        fn abi_encoded_size(&self) -> usize {
            match self {
                Self::deregisterOperator(inner) => {
                    <deregisterOperatorCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getLastCheckpointOperatorWeight(inner) => {
                    <getLastCheckpointOperatorWeightCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getLastCheckpointThresholdWeight(inner) => {
                    <getLastCheckpointThresholdWeightCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getLastCheckpointThresholdWeightAtBlock(inner) => {
                    <getLastCheckpointThresholdWeightAtBlockCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getLastCheckpointTotalWeight(inner) => {
                    <getLastCheckpointTotalWeightCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getLastCheckpointTotalWeightAtBlock(inner) => {
                    <getLastCheckpointTotalWeightAtBlockCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getLatestOperatorSigningKey(inner) => {
                    <getLatestOperatorSigningKeyCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getOperatorSigningKeyAtBlock(inner) => {
                    <getOperatorSigningKeyAtBlockCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getOperatorWeight(inner) => {
                    <getOperatorWeightCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getOperatorWeightAtBlock(inner) => {
                    <getOperatorWeightAtBlockCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::initialize(inner) => {
                    <initializeCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::isValidSignature(inner) => {
                    <isValidSignatureCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::minimumWeight(inner) => {
                    <minimumWeightCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::operatorRegistered(inner) => {
                    <operatorRegisteredCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::owner(inner) => {
                    <ownerCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::quorum(inner) => {
                    <quorumCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::registerOperatorWithSignature(inner) => {
                    <registerOperatorWithSignatureCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::renounceOwnership(inner) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::transferOwnership(inner) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::updateMinimumWeight(inner) => {
                    <updateMinimumWeightCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::updateOperatorSigningKey(inner) => {
                    <updateOperatorSigningKeyCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::updateOperators(inner) => {
                    <updateOperatorsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::updateOperatorsForQuorum(inner) => {
                    <updateOperatorsForQuorumCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::updateQuorumConfig(inner) => {
                    <updateQuorumConfigCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::updateStakeThreshold(inner) => {
                    <updateStakeThresholdCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::deregisterOperator(inner) => {
                    <deregisterOperatorCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getLastCheckpointOperatorWeight(inner) => {
                    <getLastCheckpointOperatorWeightCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getLastCheckpointThresholdWeight(inner) => {
                    <getLastCheckpointThresholdWeightCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getLastCheckpointThresholdWeightAtBlock(inner) => {
                    <getLastCheckpointThresholdWeightAtBlockCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getLastCheckpointTotalWeight(inner) => {
                    <getLastCheckpointTotalWeightCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getLastCheckpointTotalWeightAtBlock(inner) => {
                    <getLastCheckpointTotalWeightAtBlockCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getLatestOperatorSigningKey(inner) => {
                    <getLatestOperatorSigningKeyCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getOperatorSigningKeyAtBlock(inner) => {
                    <getOperatorSigningKeyAtBlockCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getOperatorWeight(inner) => {
                    <getOperatorWeightCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getOperatorWeightAtBlock(inner) => {
                    <getOperatorWeightAtBlockCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::initialize(inner) => {
                    <initializeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::isValidSignature(inner) => {
                    <isValidSignatureCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::minimumWeight(inner) => {
                    <minimumWeightCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::operatorRegistered(inner) => {
                    <operatorRegisteredCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::owner(inner) => {
                    <ownerCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::quorum(inner) => {
                    <quorumCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::registerOperatorWithSignature(inner) => {
                    <registerOperatorWithSignatureCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::renounceOwnership(inner) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::transferOwnership(inner) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::updateMinimumWeight(inner) => {
                    <updateMinimumWeightCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::updateOperatorSigningKey(inner) => {
                    <updateOperatorSigningKeyCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::updateOperators(inner) => {
                    <updateOperatorsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::updateOperatorsForQuorum(inner) => {
                    <updateOperatorsForQuorumCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::updateQuorumConfig(inner) => {
                    <updateQuorumConfigCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::updateStakeThreshold(inner) => {
                    <updateStakeThresholdCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`ECDSAStakeRegistry`](self) custom errors.
    pub enum ECDSAStakeRegistryErrors {
        #[allow(missing_docs)]
        InsufficientSignedStake(InsufficientSignedStake),
        #[allow(missing_docs)]
        InsufficientWeight(InsufficientWeight),
        #[allow(missing_docs)]
        InvalidLength(InvalidLength),
        #[allow(missing_docs)]
        InvalidQuorum(InvalidQuorum),
        #[allow(missing_docs)]
        InvalidReferenceBlock(InvalidReferenceBlock),
        #[allow(missing_docs)]
        InvalidSignature(InvalidSignature),
        #[allow(missing_docs)]
        InvalidSignedWeight(InvalidSignedWeight),
        #[allow(missing_docs)]
        InvalidThreshold(InvalidThreshold),
        #[allow(missing_docs)]
        LengthMismatch(LengthMismatch),
        #[allow(missing_docs)]
        MustUpdateAllOperators(MustUpdateAllOperators),
        #[allow(missing_docs)]
        NotSorted(NotSorted),
        #[allow(missing_docs)]
        OperatorAlreadyRegistered(OperatorAlreadyRegistered),
        #[allow(missing_docs)]
        OperatorNotRegistered(OperatorNotRegistered),
    }
    #[automatically_derived]
    impl ECDSAStakeRegistryErrors {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [37u8, 236u8, 108u8, 31u8],
            [45u8, 61u8, 246u8, 182u8],
            [66u8, 238u8, 104u8, 181u8],
            [139u8, 170u8, 87u8, 159u8],
            [148u8, 125u8, 90u8, 132u8],
            [150u8, 11u8, 65u8, 238u8],
            [168u8, 121u8, 47u8, 209u8],
            [170u8, 189u8, 90u8, 9u8],
            [186u8, 80u8, 249u8, 17u8],
            [209u8, 115u8, 87u8, 121u8],
            [225u8, 33u8, 99u8, 47u8],
            [230u8, 79u8, 24u8, 15u8],
            [255u8, 99u8, 58u8, 56u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for ECDSAStakeRegistryErrors {
        const NAME: &'static str = "ECDSAStakeRegistryErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 13usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::InsufficientSignedStake(_) => {
                    <InsufficientSignedStake as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InsufficientWeight(_) => {
                    <InsufficientWeight as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidLength(_) => {
                    <InvalidLength as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidQuorum(_) => {
                    <InvalidQuorum as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidReferenceBlock(_) => {
                    <InvalidReferenceBlock as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidSignature(_) => {
                    <InvalidSignature as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidSignedWeight(_) => {
                    <InvalidSignedWeight as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidThreshold(_) => {
                    <InvalidThreshold as alloy_sol_types::SolError>::SELECTOR
                }
                Self::LengthMismatch(_) => {
                    <LengthMismatch as alloy_sol_types::SolError>::SELECTOR
                }
                Self::MustUpdateAllOperators(_) => {
                    <MustUpdateAllOperators as alloy_sol_types::SolError>::SELECTOR
                }
                Self::NotSorted(_) => <NotSorted as alloy_sol_types::SolError>::SELECTOR,
                Self::OperatorAlreadyRegistered(_) => {
                    <OperatorAlreadyRegistered as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OperatorNotRegistered(_) => {
                    <OperatorNotRegistered as alloy_sol_types::SolError>::SELECTOR
                }
            }
        }
        #[inline]
        fn selector_at(i: usize) -> ::core::option::Option<[u8; 4]> {
            Self::SELECTORS.get(i).copied()
        }
        #[inline]
        fn valid_selector(selector: [u8; 4]) -> bool {
            Self::SELECTORS.binary_search(&selector).is_ok()
        }
        #[inline]
        #[allow(non_snake_case)]
        fn abi_decode_raw(
            selector: [u8; 4],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_SHIMS: &[fn(
                &[u8],
                bool,
            ) -> alloy_sol_types::Result<ECDSAStakeRegistryErrors>] = &[
                {
                    fn OperatorNotRegistered(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryErrors> {
                        <OperatorNotRegistered as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ECDSAStakeRegistryErrors::OperatorNotRegistered)
                    }
                    OperatorNotRegistered
                },
                {
                    fn MustUpdateAllOperators(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryErrors> {
                        <MustUpdateAllOperators as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ECDSAStakeRegistryErrors::MustUpdateAllOperators)
                    }
                    MustUpdateAllOperators
                },
                {
                    fn OperatorAlreadyRegistered(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryErrors> {
                        <OperatorAlreadyRegistered as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ECDSAStakeRegistryErrors::OperatorAlreadyRegistered)
                    }
                    OperatorAlreadyRegistered
                },
                {
                    fn InvalidSignature(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryErrors> {
                        <InvalidSignature as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ECDSAStakeRegistryErrors::InvalidSignature)
                    }
                    InvalidSignature
                },
                {
                    fn InvalidLength(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryErrors> {
                        <InvalidLength as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ECDSAStakeRegistryErrors::InvalidLength)
                    }
                    InvalidLength
                },
                {
                    fn InvalidSignedWeight(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryErrors> {
                        <InvalidSignedWeight as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ECDSAStakeRegistryErrors::InvalidSignedWeight)
                    }
                    InvalidSignedWeight
                },
                {
                    fn InsufficientWeight(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryErrors> {
                        <InsufficientWeight as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ECDSAStakeRegistryErrors::InsufficientWeight)
                    }
                    InsufficientWeight
                },
                {
                    fn InvalidThreshold(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryErrors> {
                        <InvalidThreshold as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ECDSAStakeRegistryErrors::InvalidThreshold)
                    }
                    InvalidThreshold
                },
                {
                    fn NotSorted(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryErrors> {
                        <NotSorted as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ECDSAStakeRegistryErrors::NotSorted)
                    }
                    NotSorted
                },
                {
                    fn InvalidQuorum(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryErrors> {
                        <InvalidQuorum as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ECDSAStakeRegistryErrors::InvalidQuorum)
                    }
                    InvalidQuorum
                },
                {
                    fn InsufficientSignedStake(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryErrors> {
                        <InsufficientSignedStake as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ECDSAStakeRegistryErrors::InsufficientSignedStake)
                    }
                    InsufficientSignedStake
                },
                {
                    fn InvalidReferenceBlock(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryErrors> {
                        <InvalidReferenceBlock as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ECDSAStakeRegistryErrors::InvalidReferenceBlock)
                    }
                    InvalidReferenceBlock
                },
                {
                    fn LengthMismatch(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryErrors> {
                        <LengthMismatch as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ECDSAStakeRegistryErrors::LengthMismatch)
                    }
                    LengthMismatch
                },
            ];
            let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                return Err(
                    alloy_sol_types::Error::unknown_selector(
                        <Self as alloy_sol_types::SolInterface>::NAME,
                        selector,
                    ),
                );
            };
            DECODE_SHIMS[idx](data, validate)
        }
        #[inline]
        fn abi_encoded_size(&self) -> usize {
            match self {
                Self::InsufficientSignedStake(inner) => {
                    <InsufficientSignedStake as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InsufficientWeight(inner) => {
                    <InsufficientWeight as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidLength(inner) => {
                    <InvalidLength as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InvalidQuorum(inner) => {
                    <InvalidQuorum as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InvalidReferenceBlock(inner) => {
                    <InvalidReferenceBlock as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidSignature(inner) => {
                    <InvalidSignature as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidSignedWeight(inner) => {
                    <InvalidSignedWeight as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidThreshold(inner) => {
                    <InvalidThreshold as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::LengthMismatch(inner) => {
                    <LengthMismatch as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::MustUpdateAllOperators(inner) => {
                    <MustUpdateAllOperators as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::NotSorted(inner) => {
                    <NotSorted as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::OperatorAlreadyRegistered(inner) => {
                    <OperatorAlreadyRegistered as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::OperatorNotRegistered(inner) => {
                    <OperatorNotRegistered as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::InsufficientSignedStake(inner) => {
                    <InsufficientSignedStake as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InsufficientWeight(inner) => {
                    <InsufficientWeight as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidLength(inner) => {
                    <InvalidLength as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidQuorum(inner) => {
                    <InvalidQuorum as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidReferenceBlock(inner) => {
                    <InvalidReferenceBlock as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidSignature(inner) => {
                    <InvalidSignature as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidSignedWeight(inner) => {
                    <InvalidSignedWeight as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidThreshold(inner) => {
                    <InvalidThreshold as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::LengthMismatch(inner) => {
                    <LengthMismatch as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::MustUpdateAllOperators(inner) => {
                    <MustUpdateAllOperators as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::NotSorted(inner) => {
                    <NotSorted as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::OperatorAlreadyRegistered(inner) => {
                    <OperatorAlreadyRegistered as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::OperatorNotRegistered(inner) => {
                    <OperatorNotRegistered as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`ECDSAStakeRegistry`](self) events.
    pub enum ECDSAStakeRegistryEvents {
        #[allow(missing_docs)]
        Initialized(Initialized),
        #[allow(missing_docs)]
        MinimumWeightUpdated(MinimumWeightUpdated),
        #[allow(missing_docs)]
        OperatorDeregistered(OperatorDeregistered),
        #[allow(missing_docs)]
        OperatorRegistered(OperatorRegistered),
        #[allow(missing_docs)]
        OperatorWeightUpdated(OperatorWeightUpdated),
        #[allow(missing_docs)]
        OwnershipTransferred(OwnershipTransferred),
        #[allow(missing_docs)]
        QuorumUpdated(QuorumUpdated),
        #[allow(missing_docs)]
        SigningKeyUpdate(SigningKeyUpdate),
        #[allow(missing_docs)]
        ThresholdWeightUpdated(ThresholdWeightUpdated),
        #[allow(missing_docs)]
        TotalWeightUpdated(TotalWeightUpdated),
        #[allow(missing_docs)]
        UpdateMinimumWeight(UpdateMinimumWeight),
    }
    #[automatically_derived]
    impl ECDSAStakeRegistryEvents {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 32usize]] = &[
            [
                30u8,
                164u8,
                33u8,
                134u8,
                179u8,
                5u8,
                250u8,
                55u8,
                49u8,
                4u8,
                80u8,
                217u8,
                251u8,
                135u8,
                234u8,
                30u8,
                143u8,
                12u8,
                127u8,
                68u8,
                126u8,
                119u8,
                20u8,
                121u8,
                227u8,
                178u8,
                118u8,
                52u8,
                191u8,
                232u8,
                77u8,
                193u8,
            ],
            [
                35u8,
                170u8,
                212u8,
                230u8,
                23u8,
                68u8,
                236u8,
                225u8,
                100u8,
                19u8,
                10u8,
                164u8,
                21u8,
                193u8,
                97u8,
                110u8,
                128u8,
                19u8,
                107u8,
                15u8,
                7u8,
                112u8,
                229u8,
                101u8,
                137u8,
                67u8,
                139u8,
                144u8,
                178u8,
                105u8,
                38u8,
                94u8,
            ],
            [
                49u8,
                224u8,
                173u8,
                254u8,
                199u8,
                27u8,
                204u8,
                238u8,
                55u8,
                182u8,
                232u8,
                58u8,
                144u8,
                194u8,
                254u8,
                219u8,
                23u8,
                216u8,
                241u8,
                105u8,
                63u8,
                238u8,
                134u8,
                60u8,
                71u8,
                113u8,
                231u8,
                191u8,
                226u8,
                174u8,
                213u8,
                128u8,
            ],
            [
                113u8,
                60u8,
                165u8,
                59u8,
                136u8,
                214u8,
                235u8,
                99u8,
                245u8,
                177u8,
                133u8,
                76u8,
                184u8,
                203u8,
                221u8,
                115u8,
                110u8,
                197u8,
                30u8,
                218u8,
                34u8,
                94u8,
                70u8,
                121u8,
                26u8,
                169u8,
                41u8,
                139u8,
                1u8,
                96u8,
                100u8,
                143u8,
            ],
            [
                127u8,
                38u8,
                184u8,
                63u8,
                249u8,
                110u8,
                31u8,
                43u8,
                106u8,
                104u8,
                47u8,
                19u8,
                56u8,
                82u8,
                246u8,
                121u8,
                138u8,
                9u8,
                196u8,
                101u8,
                218u8,
                149u8,
                146u8,
                20u8,
                96u8,
                206u8,
                251u8,
                56u8,
                71u8,
                64u8,
                36u8,
                152u8,
            ],
            [
                134u8,
                220u8,
                248u8,
                107u8,
                18u8,
                223u8,
                238u8,
                222u8,
                167u8,
                74u8,
                233u8,
                48u8,
                13u8,
                189u8,
                170u8,
                25u8,
                59u8,
                204u8,
                229u8,
                128u8,
                147u8,
                105u8,
                200u8,
                23u8,
                126u8,
                162u8,
                244u8,
                234u8,
                170u8,
                101u8,
                114u8,
                155u8,
            ],
            [
                136u8,
                119u8,
                13u8,
                200u8,
                98u8,
                228u8,
                122u8,
                126u8,
                213u8,
                134u8,
                144u8,
                120u8,
                87u8,
                235u8,
                27u8,
                117u8,
                228u8,
                197u8,
                255u8,
                200u8,
                183u8,
                7u8,
                199u8,
                238u8,
                16u8,
                235u8,
                116u8,
                214u8,
                136u8,
                95u8,
                229u8,
                148u8,
            ],
            [
                139u8,
                224u8,
                7u8,
                156u8,
                83u8,
                22u8,
                89u8,
                20u8,
                19u8,
                68u8,
                205u8,
                31u8,
                208u8,
                164u8,
                242u8,
                132u8,
                25u8,
                73u8,
                127u8,
                151u8,
                34u8,
                163u8,
                218u8,
                175u8,
                227u8,
                180u8,
                24u8,
                111u8,
                107u8,
                100u8,
                87u8,
                224u8,
            ],
            [
                147u8,
                36u8,
                247u8,
                229u8,
                167u8,
                192u8,
                40u8,
                136u8,
                8u8,
                166u8,
                52u8,
                204u8,
                222u8,
                68u8,
                184u8,
                233u8,
                121u8,
                103u8,
                100u8,
                116u8,
                178u8,
                46u8,
                41u8,
                238u8,
                157u8,
                213u8,
                105u8,
                181u8,
                94u8,
                121u8,
                26u8,
                75u8,
            ],
            [
                164u8,
                83u8,
                219u8,
                97u8,
                42u8,
                245u8,
                158u8,
                85u8,
                33u8,
                214u8,
                171u8,
                146u8,
                132u8,
                220u8,
                62u8,
                45u8,
                6u8,
                175u8,
                40u8,
                110u8,
                177u8,
                177u8,
                183u8,
                183u8,
                113u8,
                252u8,
                228u8,
                113u8,
                108u8,
                25u8,
                242u8,
                193u8,
            ],
            [
                208u8,
                97u8,
                22u8,
                130u8,
                82u8,
                244u8,
                65u8,
                115u8,
                54u8,
                88u8,
                240u8,
                158u8,
                77u8,
                143u8,
                91u8,
                45u8,
                153u8,
                142u8,
                212u8,
                239u8,
                36u8,
                162u8,
                187u8,
                253u8,
                108u8,
                236u8,
                165u8,
                46u8,
                161u8,
                49u8,
                80u8,
                2u8,
            ],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolEventInterface for ECDSAStakeRegistryEvents {
        const NAME: &'static str = "ECDSAStakeRegistryEvents";
        const COUNT: usize = 11usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(<Initialized as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Initialized as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::Initialized)
                }
                Some(
                    <MinimumWeightUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <MinimumWeightUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::MinimumWeightUpdated)
                }
                Some(
                    <OperatorDeregistered as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <OperatorDeregistered as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::OperatorDeregistered)
                }
                Some(
                    <OperatorRegistered as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <OperatorRegistered as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::OperatorRegistered)
                }
                Some(
                    <OperatorWeightUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <OperatorWeightUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::OperatorWeightUpdated)
                }
                Some(
                    <OwnershipTransferred as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <OwnershipTransferred as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::OwnershipTransferred)
                }
                Some(<QuorumUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <QuorumUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::QuorumUpdated)
                }
                Some(<SigningKeyUpdate as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <SigningKeyUpdate as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::SigningKeyUpdate)
                }
                Some(
                    <ThresholdWeightUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <ThresholdWeightUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::ThresholdWeightUpdated)
                }
                Some(
                    <TotalWeightUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <TotalWeightUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::TotalWeightUpdated)
                }
                Some(
                    <UpdateMinimumWeight as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <UpdateMinimumWeight as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::UpdateMinimumWeight)
                }
                _ => {
                    alloy_sol_types::private::Err(alloy_sol_types::Error::InvalidLog {
                        name: <Self as alloy_sol_types::SolEventInterface>::NAME,
                        log: alloy_sol_types::private::Box::new(
                            alloy_sol_types::private::LogData::new_unchecked(
                                topics.to_vec(),
                                data.to_vec().into(),
                            ),
                        ),
                    })
                }
            }
        }
    }
    #[automatically_derived]
    impl alloy_sol_types::private::IntoLogData for ECDSAStakeRegistryEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::Initialized(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::MinimumWeightUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OperatorDeregistered(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OperatorRegistered(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OperatorWeightUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OwnershipTransferred(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::QuorumUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::SigningKeyUpdate(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::ThresholdWeightUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::TotalWeightUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::UpdateMinimumWeight(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
            }
        }
        fn into_log_data(self) -> alloy_sol_types::private::LogData {
            match self {
                Self::Initialized(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::MinimumWeightUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OperatorDeregistered(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OperatorRegistered(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OperatorWeightUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OwnershipTransferred(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::QuorumUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::SigningKeyUpdate(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::ThresholdWeightUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::TotalWeightUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::UpdateMinimumWeight(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`ECDSAStakeRegistry`](self) contract instance.

See the [wrapper's documentation](`ECDSAStakeRegistryInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> ECDSAStakeRegistryInstance<T, P, N> {
        ECDSAStakeRegistryInstance::<T, P, N>::new(address, provider)
    }
    /**Deploys this contract using the given `provider` and constructor arguments, if any.

Returns a new instance of the contract, if the deployment was successful.

For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
    #[inline]
    pub fn deploy<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        provider: P,
        _delegationManager: alloy::sol_types::private::Address,
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<ECDSAStakeRegistryInstance<T, P, N>>,
    > {
        ECDSAStakeRegistryInstance::<T, P, N>::deploy(provider, _delegationManager)
    }
    /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
and constructor arguments, if any.

This is a simple wrapper around creating a `RawCallBuilder` with the data set to
the bytecode concatenated with the constructor's ABI-encoded arguments.*/
    #[inline]
    pub fn deploy_builder<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        provider: P,
        _delegationManager: alloy::sol_types::private::Address,
    ) -> alloy_contract::RawCallBuilder<T, P, N> {
        ECDSAStakeRegistryInstance::<
            T,
            P,
            N,
        >::deploy_builder(provider, _delegationManager)
    }
    /**A [`ECDSAStakeRegistry`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`ECDSAStakeRegistry`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct ECDSAStakeRegistryInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for ECDSAStakeRegistryInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("ECDSAStakeRegistryInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > ECDSAStakeRegistryInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`ECDSAStakeRegistry`](self) contract instance.

See the [wrapper's documentation](`ECDSAStakeRegistryInstance`) for more details.*/
        #[inline]
        pub const fn new(
            address: alloy_sol_types::private::Address,
            provider: P,
        ) -> Self {
            Self {
                address,
                provider,
                _network_transport: ::core::marker::PhantomData,
            }
        }
        /**Deploys this contract using the given `provider` and constructor arguments, if any.

Returns a new instance of the contract, if the deployment was successful.

For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
        #[inline]
        pub async fn deploy(
            provider: P,
            _delegationManager: alloy::sol_types::private::Address,
        ) -> alloy_contract::Result<ECDSAStakeRegistryInstance<T, P, N>> {
            let call_builder = Self::deploy_builder(provider, _delegationManager);
            let contract_address = call_builder.deploy().await?;
            Ok(Self::new(contract_address, call_builder.provider))
        }
        /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
and constructor arguments, if any.

This is a simple wrapper around creating a `RawCallBuilder` with the data set to
the bytecode concatenated with the constructor's ABI-encoded arguments.*/
        #[inline]
        pub fn deploy_builder(
            provider: P,
            _delegationManager: alloy::sol_types::private::Address,
        ) -> alloy_contract::RawCallBuilder<T, P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                provider,
                [
                    &BYTECODE[..],
                    &alloy_sol_types::SolConstructor::abi_encode(
                        &constructorCall {
                            _delegationManager,
                        },
                    )[..],
                ]
                    .concat()
                    .into(),
            )
        }
        /// Returns a reference to the address.
        #[inline]
        pub const fn address(&self) -> &alloy_sol_types::private::Address {
            &self.address
        }
        /// Sets the address.
        #[inline]
        pub fn set_address(&mut self, address: alloy_sol_types::private::Address) {
            self.address = address;
        }
        /// Sets the address and returns `self`.
        pub fn at(mut self, address: alloy_sol_types::private::Address) -> Self {
            self.set_address(address);
            self
        }
        /// Returns a reference to the provider.
        #[inline]
        pub const fn provider(&self) -> &P {
            &self.provider
        }
    }
    impl<T, P: ::core::clone::Clone, N> ECDSAStakeRegistryInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> ECDSAStakeRegistryInstance<T, P, N> {
            ECDSAStakeRegistryInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network_transport: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > ECDSAStakeRegistryInstance<T, P, N> {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<T, &P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
        ///Creates a new call builder for the [`deregisterOperator`] function.
        pub fn deregisterOperator(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, deregisterOperatorCall, N> {
            self.call_builder(&deregisterOperatorCall {})
        }
        ///Creates a new call builder for the [`getLastCheckpointOperatorWeight`] function.
        pub fn getLastCheckpointOperatorWeight(
            &self,
            operator: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            getLastCheckpointOperatorWeightCall,
            N,
        > {
            self.call_builder(
                &getLastCheckpointOperatorWeightCall {
                    operator,
                },
            )
        }
        ///Creates a new call builder for the [`getLastCheckpointThresholdWeight`] function.
        pub fn getLastCheckpointThresholdWeight(
            &self,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            getLastCheckpointThresholdWeightCall,
            N,
        > {
            self.call_builder(
                &getLastCheckpointThresholdWeightCall {
                },
            )
        }
        ///Creates a new call builder for the [`getLastCheckpointThresholdWeightAtBlock`] function.
        pub fn getLastCheckpointThresholdWeightAtBlock(
            &self,
            blockNumber: u32,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            getLastCheckpointThresholdWeightAtBlockCall,
            N,
        > {
            self.call_builder(
                &getLastCheckpointThresholdWeightAtBlockCall {
                    blockNumber,
                },
            )
        }
        ///Creates a new call builder for the [`getLastCheckpointTotalWeight`] function.
        pub fn getLastCheckpointTotalWeight(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getLastCheckpointTotalWeightCall, N> {
            self.call_builder(
                &getLastCheckpointTotalWeightCall {
                },
            )
        }
        ///Creates a new call builder for the [`getLastCheckpointTotalWeightAtBlock`] function.
        pub fn getLastCheckpointTotalWeightAtBlock(
            &self,
            blockNumber: u32,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            getLastCheckpointTotalWeightAtBlockCall,
            N,
        > {
            self.call_builder(
                &getLastCheckpointTotalWeightAtBlockCall {
                    blockNumber,
                },
            )
        }
        ///Creates a new call builder for the [`getLatestOperatorSigningKey`] function.
        pub fn getLatestOperatorSigningKey(
            &self,
            operator: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, getLatestOperatorSigningKeyCall, N> {
            self.call_builder(
                &getLatestOperatorSigningKeyCall {
                    operator,
                },
            )
        }
        ///Creates a new call builder for the [`getOperatorSigningKeyAtBlock`] function.
        pub fn getOperatorSigningKeyAtBlock(
            &self,
            operator: alloy::sol_types::private::Address,
            blockNumber: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, getOperatorSigningKeyAtBlockCall, N> {
            self.call_builder(
                &getOperatorSigningKeyAtBlockCall {
                    operator,
                    blockNumber,
                },
            )
        }
        ///Creates a new call builder for the [`getOperatorWeight`] function.
        pub fn getOperatorWeight(
            &self,
            operator: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, getOperatorWeightCall, N> {
            self.call_builder(&getOperatorWeightCall { operator })
        }
        ///Creates a new call builder for the [`getOperatorWeightAtBlock`] function.
        pub fn getOperatorWeightAtBlock(
            &self,
            operator: alloy::sol_types::private::Address,
            blockNumber: u32,
        ) -> alloy_contract::SolCallBuilder<T, &P, getOperatorWeightAtBlockCall, N> {
            self.call_builder(
                &getOperatorWeightAtBlockCall {
                    operator,
                    blockNumber,
                },
            )
        }
        ///Creates a new call builder for the [`initialize`] function.
        pub fn initialize(
            &self,
            _serviceManager: alloy::sol_types::private::Address,
            thresholdWeight: alloy::sol_types::private::primitives::aliases::U256,
            quorum: <IECDSAStakeRegistryTypes::Quorum as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, initializeCall, N> {
            self.call_builder(
                &initializeCall {
                    _serviceManager,
                    thresholdWeight,
                    quorum,
                },
            )
        }
        ///Creates a new call builder for the [`isValidSignature`] function.
        pub fn isValidSignature(
            &self,
            digest: alloy::sol_types::private::FixedBytes<32>,
            _signatureData: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, isValidSignatureCall, N> {
            self.call_builder(
                &isValidSignatureCall {
                    digest,
                    _signatureData,
                },
            )
        }
        ///Creates a new call builder for the [`minimumWeight`] function.
        pub fn minimumWeight(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, minimumWeightCall, N> {
            self.call_builder(&minimumWeightCall {})
        }
        ///Creates a new call builder for the [`operatorRegistered`] function.
        pub fn operatorRegistered(
            &self,
            operator: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, operatorRegisteredCall, N> {
            self.call_builder(&operatorRegisteredCall { operator })
        }
        ///Creates a new call builder for the [`owner`] function.
        pub fn owner(&self) -> alloy_contract::SolCallBuilder<T, &P, ownerCall, N> {
            self.call_builder(&ownerCall {})
        }
        ///Creates a new call builder for the [`quorum`] function.
        pub fn quorum(&self) -> alloy_contract::SolCallBuilder<T, &P, quorumCall, N> {
            self.call_builder(&quorumCall {})
        }
        ///Creates a new call builder for the [`registerOperatorWithSignature`] function.
        pub fn registerOperatorWithSignature(
            &self,
            operatorSignature: <ISignatureUtils::SignatureWithSaltAndExpiry as alloy::sol_types::SolType>::RustType,
            signingKey: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            registerOperatorWithSignatureCall,
            N,
        > {
            self.call_builder(
                &registerOperatorWithSignatureCall {
                    operatorSignature,
                    signingKey,
                },
            )
        }
        ///Creates a new call builder for the [`renounceOwnership`] function.
        pub fn renounceOwnership(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, renounceOwnershipCall, N> {
            self.call_builder(&renounceOwnershipCall {})
        }
        ///Creates a new call builder for the [`transferOwnership`] function.
        pub fn transferOwnership(
            &self,
            newOwner: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, transferOwnershipCall, N> {
            self.call_builder(&transferOwnershipCall { newOwner })
        }
        ///Creates a new call builder for the [`updateMinimumWeight`] function.
        pub fn updateMinimumWeight(
            &self,
            newMinimumWeight: alloy::sol_types::private::primitives::aliases::U256,
            operators: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
        ) -> alloy_contract::SolCallBuilder<T, &P, updateMinimumWeightCall, N> {
            self.call_builder(
                &updateMinimumWeightCall {
                    newMinimumWeight,
                    operators,
                },
            )
        }
        ///Creates a new call builder for the [`updateOperatorSigningKey`] function.
        pub fn updateOperatorSigningKey(
            &self,
            newSigningKey: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, updateOperatorSigningKeyCall, N> {
            self.call_builder(
                &updateOperatorSigningKeyCall {
                    newSigningKey,
                },
            )
        }
        ///Creates a new call builder for the [`updateOperators`] function.
        pub fn updateOperators(
            &self,
            operators: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
        ) -> alloy_contract::SolCallBuilder<T, &P, updateOperatorsCall, N> {
            self.call_builder(&updateOperatorsCall { operators })
        }
        ///Creates a new call builder for the [`updateOperatorsForQuorum`] function.
        pub fn updateOperatorsForQuorum(
            &self,
            operatorsPerQuorum: alloy::sol_types::private::Vec<
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
            >,
            _1: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, updateOperatorsForQuorumCall, N> {
            self.call_builder(
                &updateOperatorsForQuorumCall {
                    operatorsPerQuorum,
                    _1,
                },
            )
        }
        ///Creates a new call builder for the [`updateQuorumConfig`] function.
        pub fn updateQuorumConfig(
            &self,
            quorum: <IECDSAStakeRegistryTypes::Quorum as alloy::sol_types::SolType>::RustType,
            operators: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
        ) -> alloy_contract::SolCallBuilder<T, &P, updateQuorumConfigCall, N> {
            self.call_builder(
                &updateQuorumConfigCall {
                    quorum,
                    operators,
                },
            )
        }
        ///Creates a new call builder for the [`updateStakeThreshold`] function.
        pub fn updateStakeThreshold(
            &self,
            thresholdWeight: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, updateStakeThresholdCall, N> {
            self.call_builder(
                &updateStakeThresholdCall {
                    thresholdWeight,
                },
            )
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > ECDSAStakeRegistryInstance<T, P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<T, &P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
        ///Creates a new event filter for the [`Initialized`] event.
        pub fn Initialized_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, Initialized, N> {
            self.event_filter::<Initialized>()
        }
        ///Creates a new event filter for the [`MinimumWeightUpdated`] event.
        pub fn MinimumWeightUpdated_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, MinimumWeightUpdated, N> {
            self.event_filter::<MinimumWeightUpdated>()
        }
        ///Creates a new event filter for the [`OperatorDeregistered`] event.
        pub fn OperatorDeregistered_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, OperatorDeregistered, N> {
            self.event_filter::<OperatorDeregistered>()
        }
        ///Creates a new event filter for the [`OperatorRegistered`] event.
        pub fn OperatorRegistered_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, OperatorRegistered, N> {
            self.event_filter::<OperatorRegistered>()
        }
        ///Creates a new event filter for the [`OperatorWeightUpdated`] event.
        pub fn OperatorWeightUpdated_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, OperatorWeightUpdated, N> {
            self.event_filter::<OperatorWeightUpdated>()
        }
        ///Creates a new event filter for the [`OwnershipTransferred`] event.
        pub fn OwnershipTransferred_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, OwnershipTransferred, N> {
            self.event_filter::<OwnershipTransferred>()
        }
        ///Creates a new event filter for the [`QuorumUpdated`] event.
        pub fn QuorumUpdated_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, QuorumUpdated, N> {
            self.event_filter::<QuorumUpdated>()
        }
        ///Creates a new event filter for the [`SigningKeyUpdate`] event.
        pub fn SigningKeyUpdate_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, SigningKeyUpdate, N> {
            self.event_filter::<SigningKeyUpdate>()
        }
        ///Creates a new event filter for the [`ThresholdWeightUpdated`] event.
        pub fn ThresholdWeightUpdated_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, ThresholdWeightUpdated, N> {
            self.event_filter::<ThresholdWeightUpdated>()
        }
        ///Creates a new event filter for the [`TotalWeightUpdated`] event.
        pub fn TotalWeightUpdated_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, TotalWeightUpdated, N> {
            self.event_filter::<TotalWeightUpdated>()
        }
        ///Creates a new event filter for the [`UpdateMinimumWeight`] event.
        pub fn UpdateMinimumWeight_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, UpdateMinimumWeight, N> {
            self.event_filter::<UpdateMinimumWeight>()
        }
    }
}
