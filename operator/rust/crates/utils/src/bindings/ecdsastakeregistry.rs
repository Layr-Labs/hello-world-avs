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
        pub strategies:
            alloy::sol_types::private::Vec<<StrategyParams as alloy::sol_types::SolType>::RustType>,
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
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Array<StrategyParams>,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Vec<<StrategyParams as alloy::sol_types::SolType>::RustType>,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
                Self {
                    strategies: tuple.0,
                }
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
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encode_packed_to(
                    &tuple, out,
                )
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_packed_encoded_size(
                    &tuple,
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for Quorum {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> =
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> =
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for Quorum {
            const NAME: &'static str = "Quorum";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed("Quorum(StrategyParams[] strategies)")
            }
            #[inline]
            fn eip712_components(
            ) -> alloy_sol_types::private::Vec<alloy_sol_types::private::Cow<'static, str>>
            {
                let mut components = alloy_sol_types::private::Vec::with_capacity(1);
                components.push(<StrategyParams as alloy_sol_types::SolStruct>::eip712_root_type());
                components
                    .extend(<StrategyParams as alloy_sol_types::SolStruct>::eip712_components());
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
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
                <alloy::sol_types::sol_data::Array<
                    StrategyParams,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.strategies,
                    out,
                );
            }
            #[inline]
            fn encode_topic(rust: &Self::RustType) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, &mut out);
                alloy_sol_types::abi::token::WordToken(alloy_sol_types::private::keccak256(out))
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
                    <alloy::sol_types::sol_data::Uint<96> as alloy_sol_types::SolType>::tokenize(
                        &self.multiplier,
                    ),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encode_packed_to(
                    &tuple, out,
                )
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_packed_encoded_size(
                    &tuple,
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for StrategyParams {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> =
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> =
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::detokenize(token);
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
            fn eip712_components(
            ) -> alloy_sol_types::private::Vec<alloy_sol_types::private::Cow<'static, str>>
            {
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
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
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
            fn encode_topic(rust: &Self::RustType) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, &mut out);
                alloy_sol_types::abi::token::WordToken(alloy_sol_types::private::keccak256(out))
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
    pub struct IECDSAStakeRegistryTypesInstance<T, P, N = alloy_contract::private::Ethereum> {
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
        > IECDSAStakeRegistryTypesInstance<T, P, N>
    {
        /**Creates a new wrapper around an on-chain [`IECDSAStakeRegistryTypes`](self) contract instance.

        See the [wrapper's documentation](`IECDSAStakeRegistryTypesInstance`) for more details.*/
        #[inline]
        pub const fn new(address: alloy_sol_types::private::Address, provider: P) -> Self {
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
        > IECDSAStakeRegistryTypesInstance<T, P, N>
    {
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
        > IECDSAStakeRegistryTypesInstance<T, P, N>
    {
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
library ISignatureUtilsMixinTypes {
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
pub mod ISignatureUtilsMixinTypes {
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<SignatureWithSaltAndExpiry> for UnderlyingRustTuple<'_> {
            fn from(value: SignatureWithSaltAndExpiry) -> Self {
                (value.signature, value.salt, value.expiry)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for SignatureWithSaltAndExpiry {
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
        impl alloy_sol_types::private::SolTypeValue<Self> for SignatureWithSaltAndExpiry {
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
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encode_packed_to(
                    &tuple, out,
                )
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_packed_encoded_size(
                    &tuple,
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for SignatureWithSaltAndExpiry {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> =
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> =
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::detokenize(token);
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
            fn eip712_components(
            ) -> alloy_sol_types::private::Vec<alloy_sol_types::private::Cow<'static, str>>
            {
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
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
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
            fn encode_topic(rust: &Self::RustType) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, &mut out);
                alloy_sol_types::abi::token::WordToken(alloy_sol_types::private::keccak256(out))
            }
        }
    };
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`ISignatureUtilsMixinTypes`](self) contract instance.

    See the [wrapper's documentation](`ISignatureUtilsMixinTypesInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> ISignatureUtilsMixinTypesInstance<T, P, N> {
        ISignatureUtilsMixinTypesInstance::<T, P, N>::new(address, provider)
    }
    /**A [`ISignatureUtilsMixinTypes`](self) instance.

    Contains type-safe methods for interacting with an on-chain instance of the
    [`ISignatureUtilsMixinTypes`](self) contract located at a given `address`, using a given
    provider `P`.

    If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
    documentation on how to provide it), the `deploy` and `deploy_builder` methods can
    be used to deploy a new instance of the contract.

    See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct ISignatureUtilsMixinTypesInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for ISignatureUtilsMixinTypesInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("ISignatureUtilsMixinTypesInstance")
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
        > ISignatureUtilsMixinTypesInstance<T, P, N>
    {
        /**Creates a new wrapper around an on-chain [`ISignatureUtilsMixinTypes`](self) contract instance.

        See the [wrapper's documentation](`ISignatureUtilsMixinTypesInstance`) for more details.*/
        #[inline]
        pub const fn new(address: alloy_sol_types::private::Address, provider: P) -> Self {
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
    impl<T, P: ::core::clone::Clone, N> ISignatureUtilsMixinTypesInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> ISignatureUtilsMixinTypesInstance<T, P, N> {
            ISignatureUtilsMixinTypesInstance {
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
        > ISignatureUtilsMixinTypesInstance<T, P, N>
    {
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
        > ISignatureUtilsMixinTypesInstance<T, P, N>
    {
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

library ISignatureUtilsMixinTypes {
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
    function registerOperatorWithSignature(ISignatureUtilsMixinTypes.SignatureWithSaltAndExpiry memory operatorSignature, address signingKey) external;
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
        "internalType": "struct ISignatureUtilsMixinTypes.SignatureWithSaltAndExpiry",
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
    ///0x60a034606657601f61254a38819003918201601f19168301916001600160401b03831184841017606a57808492602094604052833981010312606657516001600160a01b03811681036066576080526040516124cb908161007f8239608051816113080152f35b5f80fd5b634e487b7160e01b5f52604160045260245ffdfe60806040526004361015610011575f80fd5b5f3560e01c8062cf2ab5146101a35780630dba33941461019e5780631626ba7e146101995780631703a018146101945780631e4cd85e1461018f578063314f3a491461018a5780633b242e4a146101855780633d5611f61461018057806340bf2fb71461017b5780635140a548146101765780635e1042e8146101715780635ef533291461016c578063696255be14610167578063715018a614610162578063743c31f41461015d578063857dc190146101585780638da5cb5b14610153578063955f2d901461014e57806398ec1ac914610149578063a2ce5fd114610144578063ab1189951461013f578063b933fa741461013a578063dec5d1f614610135578063ec7fbb31146101305763f2fde38b1461012b575f80fd5b610e04565b610dc4565b610d60565b610cf1565b610bdf565b610ad4565b610aad565b610a5f565b610a37565b61091d565b6108d2565b610877565b6107ec565b6107c8565b610785565b6106bf565b6106a2565b610618565b6105d0565b6105a5565b610578565b61052c565b6103b5565b61031f565b6102d7565b634e487b7160e01b5f52604160045260245ffd5b602081019081106001600160401b038211176101d757604052565b6101a8565b606081019081106001600160401b038211176101d757604052565b604081019081106001600160401b038211176101d757604052565b90601f801991011681019081106001600160401b038211176101d757604052565b60405190610242604083610212565b565b6001600160401b0381116101d75760051b60200190565b6001600160a01b0381160361026c57565b5f80fd5b9080601f8301121561026c57813561028781610244565b926102956040519485610212565b81845260208085019260051b82010192831161026c57602001905b8282106102bd5750505090565b6020809183356102cc8161025b565b8152019101906102b0565b3461026c57602036600319011261026c576004356001600160401b03811161026c5761030a61030f913690600401610270565b611464565b005b63ffffffff81160361026c57565b3461026c57602036600319011261026c57602061034c63ffffffff60043561034681610311565b166114c1565b604051908152f35b6001600160401b0381116101d757601f01601f191660200190565b81601f8201121561026c5780359061038682610354565b926103946040519485610212565b8284526020838301011161026c57815f926020809301838601378301015290565b3461026c57604036600319011261026c576004356024356001600160401b03811161026c576103e890369060040161036f565b9081518201916060816020850194031261026c5760208101516001600160401b03811161026c57810183603f8201121561026c5760208101519061042b82610244565b916104396040519384610212565b8083526020808085019260051b840101019186831161026c57604001905b8282106104af5750505060408201516001600160401b03811161026c57606061048b61049896602061049294870101610e95565b9301610f4d565b92611756565b604051630b135d3f60e11b815280602081015b0390f35b6020809183516104be8161025b565b815201910190610457565b5f91031261026c57565b602060408184019251938281528451809452019201905f5b8181106104f85750505090565b825180516001600160a01b031685526020908101516001600160601b031681860152604090940193909201916001016104eb565b3461026c575f36600319011261026c57606060405161054a816101bc565b526104ab60405161055a816101bc565b610562610f58565b81526040519182916020835260208301906104d3565b3461026c57602036600319011261026c57602061034c63ffffffff60043561059f81610311565b166115e8565b3461026c575f36600319011261026c5760206001600160e01b036105c7611839565b16604051908152f35b3461026c57602036600319011261026c576004356105ed8161025b565b6001600160a01b03165f908152606d602090815260409091206001600160e01b03906105c79061187d565b3461026c57604036600319011261026c576004356001600160401b03811161026c576060600319823603011261026c57604051610654816101dc565b81600401356001600160401b03811161026c5761030f9261067d6044926004369184010161036f565b835260248101356020840152013560408201526024359061069d8261025b565b610fcf565b3461026c575f36600319011261026c576020606754604051908152f35b3461026c57604036600319011261026c576004356001600160401b03811161026c573660238201121561026c5780600401356106fa81610244565b916107086040519384610212565b8183526024602084019260051b8201019036821161026c5760248101925b82841061075657602435856001600160401b03821161026c5761075061030f92369060040161036f565b50611109565b83356001600160401b03811161026c5760209161077a839260243691870101610270565b815201930192610726565b3461026c57604036600319011261026c5760206004356107a48161025b565b6001600160a01b039081165f908152606a8352604090206105c790602435906116b5565b3461026c57602036600319011261026c5761030f6004356107e7611917565b61196f565b3461026c57604036600319011261026c576024356004356001600160401b03821161026c577f713ca53b88d6eb63f5b1854cb8cbdd736ec51eda225e46791aa9298b0160648f61084361030f933690600401610270565b9161084c611917565b6067548160675561086f6040519283928360209093929193604081019481520152565b0390a1611464565b3461026c575f36600319011261026c5761088f611917565b603380546001600160a01b031981169091555f906001600160a01b03167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e08280a3005b3461026c57602036600319011261026c576004356108ef8161025b565b335f52606e60205260ff60405f2054161561090e5761030f9033611a69565b6325ec6c1f60e01b5f5260045ffd5b3461026c575f36600319011261026c57335f908152606e6020526040902061094f9061094b905b5460ff1690565b1590565b61090e57610966610961606554611afa565b606555565b335f908152606e60205260409020805460ff1916905561098d61098833611d3e565b611e6f565b50506068546109b2906109a6906001600160a01b031681565b6001600160a01b031690565b803b1561026c576040516351b27a6d60e11b8152336004820152905f908290602490829084905af18015610a3257610a18575b6068546001600160a01b0316337f31e0adfec71bccee37b6e83a90c2fedb17d8f1693fee863c4771e7bfe2aed5805f80a3005b80610a265f610a2c93610212565b806104c9565b5f6109e5565b611232565b3461026c575f36600319011261026c576033546040516001600160a01b039091168152602090f35b3461026c57604036600319011261026c57602061034c600435610a818161025b565b60243590610a8e82610311565b60018060a01b03165f52606d835263ffffffff60405f209116906116b5565b3461026c57602036600319011261026c57602061034c600435610acf8161025b565b61127f565b3461026c57602036600319011261026c57600435610af18161025b565b60018060a01b03165f52606a602052602060018060a01b036105c760405f2061187d565b919060208382031261026c57604051610b2d816101bc565b80938035906001600160401b03821161026c570182601f8201121561026c57803590610b5882610244565b93610b666040519586610212565b82855260208086019360061b8301019181831161026c57602001925b828410610b90575050505052565b60408483031261026c5760405190610ba7826101f7565b8435610bb28161025b565b82526020850135906001600160601b038216820361026c5782602092836040950152815201930192610b82565b3461026c57606036600319011261026c57600435610bfc8161025b565b6044356024356001600160401b03821161026c57610c21610c69923690600401610b15565b905f5493610c4f610c3961094b8760ff9060081c1690565b80968197610ce3575b8115610cc3575b506113e6565b84610c60600160ff195f5416175f55565b610cac57611b66565b610c6f57005b610c7d61ff00195f54165f55565b604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb384740249890602090a1005b610cbe61010061ff00195f5416175f55565b611b66565b303b15915081610cd5575b505f610c49565b60ff1660011490505f610cce565b600160ff8216109150610c42565b3461026c575f36600319011261026c57606c5480610d20575060205f5b6040516001600160e01b039091168152f35b805f19810111610d5b57606c5f527f2b4a51ab505fc96a0952efda2ba61bcd3078d4c02c39a186ec16f21883fbe0150154602090811c610d0e565b61123d565b3461026c57604036600319011261026c576004356001600160401b03811161026c57610d90903690600401610b15565b602435906001600160401b03821161026c5761030a610db661030f933690600401610270565b91610dbf611917565b611c47565b3461026c57602036600319011261026c57600435610de18161025b565b60018060a01b03165f52606e602052602060ff60405f2054166040519015158152f35b3461026c57602036600319011261026c57600435610e218161025b565b610e29611917565b6001600160a01b03811615610e415761030f90611a21565b60405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b6064820152608490fd5b9080601f8301121561026c57815191610ead83610244565b92610ebb6040519485610212565b80845260208085019160051b8301019183831161026c5760208101915b838310610ee757505050505090565b82516001600160401b03811161026c57820185603f8201121561026c57602081015191610f1383610354565b610f206040519182610212565b838152604083850101881061026c575f602085819660408397018386015e83010152815201920191610ed8565b519061024282610311565b60665490610f6582610244565b91610f736040519384610212565b80835260665f9081525f5160206124765f395f51905f52602085015b838310610f9c5750505050565b600160208192604051610fae816101f7565b8554848060a01b038116825260a01c83820152815201920192019190610f8f565b335f908152606e60205260409020909190610fe990610944565b6110cd5761103290610fff6109616065546118a7565b335f908152606e6020526040902061101e90805460ff19166001179055565b61102a61098833611d3e565b505033611a69565b606854611049906109a6906001600160a01b031681565b803b1561026c57604051639926ee7d60e01b8152915f9183918290849082906110769033600484016118d9565b03925af18015610a32576110b9575b506068546001600160a01b0316337fa453db612af59e5521d6ab9284dc3e2d06af286eb1b1b7b771fce4716c19f2c15f80a3565b80610a265f6110c793610212565b5f611085565b6342ee68b560e01b5f5260045ffd5b634e487b7160e01b5f52603260045260245ffd5b80518210156111045760209160051b010190565b6110dc565b80511561110457602001518051606554036111275761024290611464565b63169efb5b60e11b5f5260045ffd5b9061114082610244565b61114d6040519182610212565b828152809261115e601f1991610244565b0190602036910137565b60208183031261026c578051906001600160401b03821161026c57019080601f8301121561026c57815161119b81610244565b926111a96040519485610212565b81845260208085019260051b82010192831161026c57602001905b8282106111d15750505090565b81518152602091820191016111c4565b6060602091604081019360018060a01b031681526040838201528451809452019201905f5b8181106112135750505090565b82516001600160a01b0316845260209384019390920191600101611206565b6040513d5f823e3d90fd5b634e487b7160e01b5f52601160045260245ffd5b81810292918115918404141715610d5b57565b9060018201809211610d5b57565b91908201809211610d5b57565b611287610f58565b915f906112948451611136565b915f5b85518110156112e057806112da6112c16112b36001948a6110f0565b51516001600160a01b031690565b6112cb83886110f0565b6001600160a01b039091169052565b01611297565b50915f9061130492946040519384928392639004134760e01b8452600484016111e1565b03817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa908115610a32575f916113c4575b505f915b84518310156113a25761139a60019161139461136086866110f0565b5161138e61138260206113738a8d6110f0565b5101516001600160601b031690565b6001600160601b031690565b90611251565b90611272565b920191611344565b9150506113b3919250612710900490565b60675481106113bf5790565b505f90565b6113e091503d805f833e6113d88183610212565b810190611168565b5f611340565b156113ed57565b60405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b6064820152608490fd5b9190915f8382019384129112908015821691151617610d5b57565b905f805b83518210156114a25760019061149a906114946001600160a01b0361148d86896110f0565b5116611d3e565b90611449565b910190611468565b90506114af919250611e6f565b5050565b5f19810191908211610d5b57565b438110156115a4576114d290611ef4565b606b549063ffffffff165f5b82811061153f575050806114fb57505f5b6001600160e01b031690565b61153361150a61153a926114b3565b606b5f527fbd43cb8ece8cd1863bcd6082d65c5b0d25665b1ce17980f0da43c0ed545f98b40190565b5460201c90565b6114ef565b90918082169080831860011c8201809211610d5b57606b5f527fbd43cb8ece8cd1863bcd6082d65c5b0d25665b1ce17980f0da43c0ed545f98b482015463ffffffff168410156115925750915b906114de565b92915061159e90611264565b9061158c565b606460405162461bcd60e51b815260206004820152602060248201527f436865636b706f696e74733a20626c6f636b206e6f7420796574206d696e65646044820152fd5b438110156115a4576115f990611ef4565b606c549063ffffffff165f5b8281106116505750508061161857505f90565b61153361162761153a926114b3565b606c5f527f2b4a51ab505fc96a0952efda2ba61bcd3078d4c02c39a186ec16f21883fbe0160190565b90918082169080831860011c8201809211610d5b57606c5f527f2b4a51ab505fc96a0952efda2ba61bcd3078d4c02c39a186ec16f21883fbe01682015463ffffffff168410156116a35750915b90611605565b9291506116af90611264565b9061169d565b90438110156115a4576116c790611ef4565b81549063ffffffff165f5b828110611702575050806116e757505f919050565b61153a916116f7611533926114b3565b905f5260205f200190565b90918082169080831860011c8201809211610d5b57845f528363ffffffff6117348460205f200163ffffffff90541690565b1611156117445750915b906116d2565b92915061175090611264565b9061173e565b939291938151905f955f938151840361182a57831561181b579291905f935b83851061178c5750505050506102429293506120e0565b909192939460018060a01b036117a287846110f0565b51169863ffffffff8816914383101561180c576117f7611801926117e58d600196815f52606a6020526117de888060a01b039160405f206116b5565b1692611f5c565b876117f08b896110f0565b5191611f80565b611394898c6120b5565b950193929190611775565b63e64f180f60e01b5f5260045ffd5b63251f56a160e21b5f5260045ffd5b631fec674760e31b5f5260045ffd5b606b548061184657505f90565b805f19810111610d5b57606b5f527fbd43cb8ece8cd1863bcd6082d65c5b0d25665b1ce17980f0da43c0ed545f98b3015460201c90565b8054908161188b5750505f90565b815f19810111610d5b575f525f199060205f2001015460201c90565b5f198114610d5b5760010190565b805180835260209291819084018484015e5f828201840152601f01601f1916010190565b9060018060a01b031681526040602082015260806040611904845160608386015260a08501906118b5565b9360208101516060850152015191015290565b6033546001600160a01b0316330361192b57565b606460405162461bcd60e51b815260206004820152602060248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152fd5b61197843611ef4565b6001600160e01b0382116119cc577f9324f7e5a7c0288808a634ccde44b8e979676474b22e29ee9dd569b55e791a4b916020916119c1906001600160e01b03831690606c6122a0565b5050604051908152a1565b60405162461bcd60e51b815260206004820152602760248201527f53616665436173743a2076616c756520646f65736e27742066697420696e20326044820152663234206269747360c81b6064820152608490fd5b603380546001600160a01b039283166001600160a01b0319821681179092559091167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e05f80a3565b6001600160a01b039081165f818152606a6020526040902090929190611a8e9061187d565b6001600160a01b0390921692911690828214611af557805f52606a602052611ab98360405f20612151565b50506040516001600160a01b03909216825243917fd061168252f441733658f09e4d8f5b2d998ed4ef24a2bbfd6ceca52ea131500290602090a4565b505050565b8015610d5b575f190190565b15611b0d57565b60405162461bcd60e51b815260206004820152602b60248201527f496e697469616c697a61626c653a20636f6e7472616374206973206e6f74206960448201526a6e697469616c697a696e6760a81b6064820152608490fd5b611b9f9291610dbf91611b7f60ff5f5460081c16611b06565b60018060a01b03166001600160601b0360a01b606854161760685561196f565b611bb860ff5f5460081c16611bb381611b06565b611b06565b61024233611a21565b606654600160401b8110156101d757600181016066556066548110156111045760665f5281516020929092015160a01b6001600160a01b0319166001600160a01b0392909216919091175f5160206124765f395f51905f5290910155565b9091611c36611c44936040845260408401906104d3565b9160208184039101526104d3565b90565b611c508161217b565b15611d175760405191611c62836101bc565b611c6a610f58565b83526066545f60665580611cdd575b505f5b82518051821015611ca35790611c9d611c97826001946110f0565b51611bc1565b01611c7c565b505091907f23aad4e61744ece164130aa415c1616e80136b0f0770e56589438b90b269265e91611cd860405192839283611c1f565b0390a1565b60665f525f5160206124765f395f51905f52015f5160206124765f395f51905f525b818110611d0c5750611c79565b5f8155600101611cff565b63d173577960e01b5f5260045ffd5b81810392915f138015828513169184121617610d5b57565b6001600160a01b0381165f818152606d602052604081209092908390611d639061187d565b60018060e01b03169384611d9260ff611d8c8660018060a01b03165f52606e60205260405f2090565b54161590565b15611e0757611da091611d26565b938415611e0057611dea611de57f88770dc862e47a7ed586907857eb1b75e4c5ffc8b707c7ee10eb74d6885fe5949460018060a01b03165f52606d60205260405f2090565b61212c565b50505b604080519182526020820192909252a290565b5050505090565b505050611e138161127f565b611e1d8482611d26565b938415611e0057611e6882611e637f88770dc862e47a7ed586907857eb1b75e4c5ffc8b707c7ee10eb74d6885fe5949560018060a01b03165f52606d60205260405f2090565b612151565b5050611ded565b90611e8b6001600160e01b03611e83611839565b169283611449565b90611e9543611ef4565b6001600160e01b0383116119cc57611eb9906001600160e01b03841690606b6122a0565b505060408051848152602081018490527f86dcf86b12dfeedea74ae9300dbdaa193bcce5809369c8177ea2f4eaaa65729b9181908101611cd8565b63ffffffff8111611f085763ffffffff1690565b60405162461bcd60e51b815260206004820152602660248201527f53616665436173743a2076616c756520646f65736e27742066697420696e203360448201526532206269747360d01b6064820152608490fd5b6001600160a01b0391821691161015611f7157565b63ba50f91160e01b5f5260045ffd5b919091611f8d82846123b6565b93909260058510156120a157611fb69415938461208b575b508315611fcb575b50505015151590565b611fbc57565b638baa579f60e01b5f5260045ffd5b5f9350906120036120118594936040519283916020830195630b135d3f60e11b875260248401526040604484015260648301906118b5565b03601f198101835282610212565b51915afa3d15612084573d61202581610354565b906120336040519283610212565b81523d5f602083013e5b81612076575b81612052575b505f8080611fad565b905061206f630b135d3f60e11b91602080825183010191016123eb565b145f612049565b905060208151101590612043565b606061203d565b6001600160a01b0384811691161493505f611fa5565b634e487b7160e01b5f52602160045260245ffd5b9063ffffffff164381101561180c57611c449160018060a01b03165f52606d60205260405f206116b5565b9063ffffffff164381101561180c576120f8816114c1565b821161211d57612107906115e8565b1161210e57565b63e121632f60e01b5f5260045ffd5b634b05a0f760e11b5f5260045ffd5b5f6121409161213a43611ef4565b906122a0565b6001600160e01b0391821692911690565b61215a43611ef4565b6001600160e01b0383116119cc57612140926001600160e01b0316916122a0565b515f919082805b82518210156121d15761219b6109a66112b384866110f0565b906001600160a01b0380831691161015611f71576121c860019195611394611382602061137387896110f0565b91019093612182565b5050509061271014155f146121e4575f90565b600190565b805490600160401b8210156101d75760018201808255821015611104575f9081526020902001815160209283015190921b63ffffffff191663ffffffff92909216919091179055565b9060405161223f816101f7565b602081935463ffffffff81168352811c910152565b1561225b57565b60405162461bcd60e51b815260206004820152601b60248201527f436865636b706f696e743a2064656372656173696e67206b65797300000000006044820152606490fd5b909291928382548015155f1461238c579260209291846122d86122d36122c8612352986114b3565b855f5260205f200190565b612232565b9363ffffffff6122fd6122ef875163ffffffff1690565b828416928391161115612254565b61231761230e875163ffffffff1690565b63ffffffff1690565b036123565750612344926116f761232d926114b3565b9063ffffffff82549181199060201b169116179055565b01516001600160e01b031690565b9190565b91505061238791612374612368610233565b63ffffffff9093168352565b6001600160e01b038816828601526121e9565b612344565b50506123b19161239d612368610233565b6001600160e01b03851660208301526121e9565b5f9190565b9060418151145f146123e2576123de91602082015190606060408401519301515f1a906123fa565b9091565b50505f90600290565b9081602091031261026c575190565b7f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a0841161246a576020935f9360ff60809460405194855216868401526040830152606082015282805260015afa15610a32575f516001600160a01b0381161561246257905f90565b505f90600190565b505050505f9060039056fe46501879b8ca8525e8c2fd519e2fbfcfa2ebea26501294aa02cbfcfb12e94354a26469706673582212200bd934ca0c33158a634e91f8eba933600d6993aa31a2669352d7969c54ecbb8e64736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\xA04`fW`\x1Fa%J8\x81\x90\x03\x91\x82\x01`\x1F\x19\x16\x83\x01\x91`\x01`\x01`@\x1B\x03\x83\x11\x84\x84\x10\x17`jW\x80\x84\x92` \x94`@R\x839\x81\x01\x03\x12`fWQ`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03`fW`\x80R`@Qa$\xCB\x90\x81a\0\x7F\x829`\x80Q\x81a\x13\x08\x01R\xF3[_\x80\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD\xFE`\x80`@R`\x046\x10\x15a\0\x11W_\x80\xFD[_5`\xE0\x1C\x80b\xCF*\xB5\x14a\x01\xA3W\x80c\r\xBA3\x94\x14a\x01\x9EW\x80c\x16&\xBA~\x14a\x01\x99W\x80c\x17\x03\xA0\x18\x14a\x01\x94W\x80c\x1EL\xD8^\x14a\x01\x8FW\x80c1O:I\x14a\x01\x8AW\x80c;$.J\x14a\x01\x85W\x80c=V\x11\xF6\x14a\x01\x80W\x80c@\xBF/\xB7\x14a\x01{W\x80cQ@\xA5H\x14a\x01vW\x80c^\x10B\xE8\x14a\x01qW\x80c^\xF53)\x14a\x01lW\x80cibU\xBE\x14a\x01gW\x80cqP\x18\xA6\x14a\x01bW\x80ct<1\xF4\x14a\x01]W\x80c\x85}\xC1\x90\x14a\x01XW\x80c\x8D\xA5\xCB[\x14a\x01SW\x80c\x95_-\x90\x14a\x01NW\x80c\x98\xEC\x1A\xC9\x14a\x01IW\x80c\xA2\xCE_\xD1\x14a\x01DW\x80c\xAB\x11\x89\x95\x14a\x01?W\x80c\xB93\xFAt\x14a\x01:W\x80c\xDE\xC5\xD1\xF6\x14a\x015W\x80c\xEC\x7F\xBB1\x14a\x010Wc\xF2\xFD\xE3\x8B\x14a\x01+W_\x80\xFD[a\x0E\x04V[a\r\xC4V[a\r`V[a\x0C\xF1V[a\x0B\xDFV[a\n\xD4V[a\n\xADV[a\n_V[a\n7V[a\t\x1DV[a\x08\xD2V[a\x08wV[a\x07\xECV[a\x07\xC8V[a\x07\x85V[a\x06\xBFV[a\x06\xA2V[a\x06\x18V[a\x05\xD0V[a\x05\xA5V[a\x05xV[a\x05,V[a\x03\xB5V[a\x03\x1FV[a\x02\xD7V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[` \x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x01\xD7W`@RV[a\x01\xA8V[``\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x01\xD7W`@RV[`@\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x01\xD7W`@RV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x01\xD7W`@RV[`@Q\x90a\x02B`@\x83a\x02\x12V[V[`\x01`\x01`@\x1B\x03\x81\x11a\x01\xD7W`\x05\x1B` \x01\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x03a\x02lWV[_\x80\xFD[\x90\x80`\x1F\x83\x01\x12\x15a\x02lW\x815a\x02\x87\x81a\x02DV[\x92a\x02\x95`@Q\x94\x85a\x02\x12V[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x02lW` \x01\x90[\x82\x82\x10a\x02\xBDWPPP\x90V[` \x80\x91\x835a\x02\xCC\x81a\x02[V[\x81R\x01\x91\x01\x90a\x02\xB0V[4a\x02lW` 6`\x03\x19\x01\x12a\x02lW`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x02lWa\x03\na\x03\x0F\x916\x90`\x04\x01a\x02pV[a\x14dV[\0[c\xFF\xFF\xFF\xFF\x81\x16\x03a\x02lWV[4a\x02lW` 6`\x03\x19\x01\x12a\x02lW` a\x03Lc\xFF\xFF\xFF\xFF`\x045a\x03F\x81a\x03\x11V[\x16a\x14\xC1V[`@Q\x90\x81R\xF3[`\x01`\x01`@\x1B\x03\x81\x11a\x01\xD7W`\x1F\x01`\x1F\x19\x16` \x01\x90V[\x81`\x1F\x82\x01\x12\x15a\x02lW\x805\x90a\x03\x86\x82a\x03TV[\x92a\x03\x94`@Q\x94\x85a\x02\x12V[\x82\x84R` \x83\x83\x01\x01\x11a\x02lW\x81_\x92` \x80\x93\x01\x83\x86\x017\x83\x01\x01R\x90V[4a\x02lW`@6`\x03\x19\x01\x12a\x02lW`\x045`$5`\x01`\x01`@\x1B\x03\x81\x11a\x02lWa\x03\xE8\x906\x90`\x04\x01a\x03oV[\x90\x81Q\x82\x01\x91``\x81` \x85\x01\x94\x03\x12a\x02lW` \x81\x01Q`\x01`\x01`@\x1B\x03\x81\x11a\x02lW\x81\x01\x83`?\x82\x01\x12\x15a\x02lW` \x81\x01Q\x90a\x04+\x82a\x02DV[\x91a\x049`@Q\x93\x84a\x02\x12V[\x80\x83R` \x80\x80\x85\x01\x92`\x05\x1B\x84\x01\x01\x01\x91\x86\x83\x11a\x02lW`@\x01\x90[\x82\x82\x10a\x04\xAFWPPP`@\x82\x01Q`\x01`\x01`@\x1B\x03\x81\x11a\x02lW``a\x04\x8Ba\x04\x98\x96` a\x04\x92\x94\x87\x01\x01a\x0E\x95V[\x93\x01a\x0FMV[\x92a\x17VV[`@Qc\x0B\x13]?`\xE1\x1B\x81R\x80` \x81\x01[\x03\x90\xF3[` \x80\x91\x83Qa\x04\xBE\x81a\x02[V[\x81R\x01\x91\x01\x90a\x04WV[_\x91\x03\x12a\x02lWV[` `@\x81\x84\x01\x92Q\x93\x82\x81R\x84Q\x80\x94R\x01\x92\x01\x90_[\x81\x81\x10a\x04\xF8WPPP\x90V[\x82Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x85R` \x90\x81\x01Q`\x01`\x01``\x1B\x03\x16\x81\x86\x01R`@\x90\x94\x01\x93\x90\x92\x01\x91`\x01\x01a\x04\xEBV[4a\x02lW_6`\x03\x19\x01\x12a\x02lW```@Qa\x05J\x81a\x01\xBCV[Ra\x04\xAB`@Qa\x05Z\x81a\x01\xBCV[a\x05ba\x0FXV[\x81R`@Q\x91\x82\x91` \x83R` \x83\x01\x90a\x04\xD3V[4a\x02lW` 6`\x03\x19\x01\x12a\x02lW` a\x03Lc\xFF\xFF\xFF\xFF`\x045a\x05\x9F\x81a\x03\x11V[\x16a\x15\xE8V[4a\x02lW_6`\x03\x19\x01\x12a\x02lW` `\x01`\x01`\xE0\x1B\x03a\x05\xC7a\x189V[\x16`@Q\x90\x81R\xF3[4a\x02lW` 6`\x03\x19\x01\x12a\x02lW`\x045a\x05\xED\x81a\x02[V[`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`m` \x90\x81R`@\x90\x91 `\x01`\x01`\xE0\x1B\x03\x90a\x05\xC7\x90a\x18}V[4a\x02lW`@6`\x03\x19\x01\x12a\x02lW`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x02lW```\x03\x19\x826\x03\x01\x12a\x02lW`@Qa\x06T\x81a\x01\xDCV[\x81`\x04\x015`\x01`\x01`@\x1B\x03\x81\x11a\x02lWa\x03\x0F\x92a\x06}`D\x92`\x046\x91\x84\x01\x01a\x03oV[\x83R`$\x81\x015` \x84\x01R\x015`@\x82\x01R`$5\x90a\x06\x9D\x82a\x02[V[a\x0F\xCFV[4a\x02lW_6`\x03\x19\x01\x12a\x02lW` `gT`@Q\x90\x81R\xF3[4a\x02lW`@6`\x03\x19\x01\x12a\x02lW`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x02lW6`#\x82\x01\x12\x15a\x02lW\x80`\x04\x015a\x06\xFA\x81a\x02DV[\x91a\x07\x08`@Q\x93\x84a\x02\x12V[\x81\x83R`$` \x84\x01\x92`\x05\x1B\x82\x01\x01\x906\x82\x11a\x02lW`$\x81\x01\x92[\x82\x84\x10a\x07VW`$5\x85`\x01`\x01`@\x1B\x03\x82\x11a\x02lWa\x07Pa\x03\x0F\x926\x90`\x04\x01a\x03oV[Pa\x11\tV[\x835`\x01`\x01`@\x1B\x03\x81\x11a\x02lW` \x91a\x07z\x83\x92`$6\x91\x87\x01\x01a\x02pV[\x81R\x01\x93\x01\x92a\x07&V[4a\x02lW`@6`\x03\x19\x01\x12a\x02lW` `\x045a\x07\xA4\x81a\x02[V[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16_\x90\x81R`j\x83R`@\x90 a\x05\xC7\x90`$5\x90a\x16\xB5V[4a\x02lW` 6`\x03\x19\x01\x12a\x02lWa\x03\x0F`\x045a\x07\xE7a\x19\x17V[a\x19oV[4a\x02lW`@6`\x03\x19\x01\x12a\x02lW`$5`\x045`\x01`\x01`@\x1B\x03\x82\x11a\x02lW\x7Fq<\xA5;\x88\xD6\xEBc\xF5\xB1\x85L\xB8\xCB\xDDsn\xC5\x1E\xDA\"^Fy\x1A\xA9)\x8B\x01`d\x8Fa\x08Ca\x03\x0F\x936\x90`\x04\x01a\x02pV[\x91a\x08La\x19\x17V[`gT\x81`gUa\x08o`@Q\x92\x83\x92\x83` \x90\x93\x92\x91\x93`@\x81\x01\x94\x81R\x01RV[\x03\x90\xA1a\x14dV[4a\x02lW_6`\x03\x19\x01\x12a\x02lWa\x08\x8Fa\x19\x17V[`3\x80T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16\x90\x91U_\x90`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x82\x80\xA3\0[4a\x02lW` 6`\x03\x19\x01\x12a\x02lW`\x045a\x08\xEF\x81a\x02[V[3_R`n` R`\xFF`@_ T\x16\x15a\t\x0EWa\x03\x0F\x903a\x1AiV[c%\xECl\x1F`\xE0\x1B_R`\x04_\xFD[4a\x02lW_6`\x03\x19\x01\x12a\x02lW3_\x90\x81R`n` R`@\x90 a\tO\x90a\tK\x90[T`\xFF\x16\x90V[\x15\x90V[a\t\x0EWa\tfa\ta`eTa\x1A\xFAV[`eUV[3_\x90\x81R`n` R`@\x90 \x80T`\xFF\x19\x16\x90Ua\t\x8Da\t\x883a\x1D>V[a\x1EoV[PP`hTa\t\xB2\x90a\t\xA6\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x80;\x15a\x02lW`@QcQ\xB2zm`\xE1\x1B\x81R3`\x04\x82\x01R\x90_\x90\x82\x90`$\x90\x82\x90\x84\x90Z\xF1\x80\x15a\n2Wa\n\x18W[`hT`\x01`\x01`\xA0\x1B\x03\x163\x7F1\xE0\xAD\xFE\xC7\x1B\xCC\xEE7\xB6\xE8:\x90\xC2\xFE\xDB\x17\xD8\xF1i?\xEE\x86<Gq\xE7\xBF\xE2\xAE\xD5\x80_\x80\xA3\0[\x80a\n&_a\n,\x93a\x02\x12V[\x80a\x04\xC9V[_a\t\xE5V[a\x122V[4a\x02lW_6`\x03\x19\x01\x12a\x02lW`3T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x02lW`@6`\x03\x19\x01\x12a\x02lW` a\x03L`\x045a\n\x81\x81a\x02[V[`$5\x90a\n\x8E\x82a\x03\x11V[`\x01\x80`\xA0\x1B\x03\x16_R`m\x83Rc\xFF\xFF\xFF\xFF`@_ \x91\x16\x90a\x16\xB5V[4a\x02lW` 6`\x03\x19\x01\x12a\x02lW` a\x03L`\x045a\n\xCF\x81a\x02[V[a\x12\x7FV[4a\x02lW` 6`\x03\x19\x01\x12a\x02lW`\x045a\n\xF1\x81a\x02[V[`\x01\x80`\xA0\x1B\x03\x16_R`j` R` `\x01\x80`\xA0\x1B\x03a\x05\xC7`@_ a\x18}V[\x91\x90` \x83\x82\x03\x12a\x02lW`@Qa\x0B-\x81a\x01\xBCV[\x80\x93\x805\x90`\x01`\x01`@\x1B\x03\x82\x11a\x02lW\x01\x82`\x1F\x82\x01\x12\x15a\x02lW\x805\x90a\x0BX\x82a\x02DV[\x93a\x0Bf`@Q\x95\x86a\x02\x12V[\x82\x85R` \x80\x86\x01\x93`\x06\x1B\x83\x01\x01\x91\x81\x83\x11a\x02lW` \x01\x92[\x82\x84\x10a\x0B\x90WPPPPRV[`@\x84\x83\x03\x12a\x02lW`@Q\x90a\x0B\xA7\x82a\x01\xF7V[\x845a\x0B\xB2\x81a\x02[V[\x82R` \x85\x015\x90`\x01`\x01``\x1B\x03\x82\x16\x82\x03a\x02lW\x82` \x92\x83`@\x95\x01R\x81R\x01\x93\x01\x92a\x0B\x82V[4a\x02lW``6`\x03\x19\x01\x12a\x02lW`\x045a\x0B\xFC\x81a\x02[V[`D5`$5`\x01`\x01`@\x1B\x03\x82\x11a\x02lWa\x0C!a\x0Ci\x926\x90`\x04\x01a\x0B\x15V[\x90_T\x93a\x0COa\x0C9a\tK\x87`\xFF\x90`\x08\x1C\x16\x90V[\x80\x96\x81\x97a\x0C\xE3W[\x81\x15a\x0C\xC3W[Pa\x13\xE6V[\x84a\x0C``\x01`\xFF\x19_T\x16\x17_UV[a\x0C\xACWa\x1BfV[a\x0CoW\0[a\x0C}a\xFF\0\x19_T\x16_UV[`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x90\xA1\0[a\x0C\xBEa\x01\0a\xFF\0\x19_T\x16\x17_UV[a\x1BfV[0;\x15\x91P\x81a\x0C\xD5W[P_a\x0CIV[`\xFF\x16`\x01\x14\x90P_a\x0C\xCEV[`\x01`\xFF\x82\x16\x10\x91Pa\x0CBV[4a\x02lW_6`\x03\x19\x01\x12a\x02lW`lT\x80a\r WP` _[`@Q`\x01`\x01`\xE0\x1B\x03\x90\x91\x16\x81R\xF3[\x80_\x19\x81\x01\x11a\r[W`l_R\x7F+JQ\xABP_\xC9j\tR\xEF\xDA+\xA6\x1B\xCD0x\xD4\xC0,9\xA1\x86\xEC\x16\xF2\x18\x83\xFB\xE0\x15\x01T` \x90\x81\x1Ca\r\x0EV[a\x12=V[4a\x02lW`@6`\x03\x19\x01\x12a\x02lW`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x02lWa\r\x90\x906\x90`\x04\x01a\x0B\x15V[`$5\x90`\x01`\x01`@\x1B\x03\x82\x11a\x02lWa\x03\na\r\xB6a\x03\x0F\x936\x90`\x04\x01a\x02pV[\x91a\r\xBFa\x19\x17V[a\x1CGV[4a\x02lW` 6`\x03\x19\x01\x12a\x02lW`\x045a\r\xE1\x81a\x02[V[`\x01\x80`\xA0\x1B\x03\x16_R`n` R` `\xFF`@_ T\x16`@Q\x90\x15\x15\x81R\xF3[4a\x02lW` 6`\x03\x19\x01\x12a\x02lW`\x045a\x0E!\x81a\x02[V[a\x0E)a\x19\x17V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a\x0EAWa\x03\x0F\x90a\x1A!V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x90\xFD[\x90\x80`\x1F\x83\x01\x12\x15a\x02lW\x81Q\x91a\x0E\xAD\x83a\x02DV[\x92a\x0E\xBB`@Q\x94\x85a\x02\x12V[\x80\x84R` \x80\x85\x01\x91`\x05\x1B\x83\x01\x01\x91\x83\x83\x11a\x02lW` \x81\x01\x91[\x83\x83\x10a\x0E\xE7WPPPPP\x90V[\x82Q`\x01`\x01`@\x1B\x03\x81\x11a\x02lW\x82\x01\x85`?\x82\x01\x12\x15a\x02lW` \x81\x01Q\x91a\x0F\x13\x83a\x03TV[a\x0F `@Q\x91\x82a\x02\x12V[\x83\x81R`@\x83\x85\x01\x01\x88\x10a\x02lW_` \x85\x81\x96`@\x83\x97\x01\x83\x86\x01^\x83\x01\x01R\x81R\x01\x92\x01\x91a\x0E\xD8V[Q\x90a\x02B\x82a\x03\x11V[`fT\x90a\x0Fe\x82a\x02DV[\x91a\x0Fs`@Q\x93\x84a\x02\x12V[\x80\x83R`f_\x90\x81R_Q` a$v_9_Q\x90_R` \x85\x01[\x83\x83\x10a\x0F\x9CWPPPPV[`\x01` \x81\x92`@Qa\x0F\xAE\x81a\x01\xF7V[\x85T\x84\x80`\xA0\x1B\x03\x81\x16\x82R`\xA0\x1C\x83\x82\x01R\x81R\x01\x92\x01\x92\x01\x91\x90a\x0F\x8FV[3_\x90\x81R`n` R`@\x90 \x90\x91\x90a\x0F\xE9\x90a\tDV[a\x10\xCDWa\x102\x90a\x0F\xFFa\ta`eTa\x18\xA7V[3_\x90\x81R`n` R`@\x90 a\x10\x1E\x90\x80T`\xFF\x19\x16`\x01\x17\x90UV[a\x10*a\t\x883a\x1D>V[PP3a\x1AiV[`hTa\x10I\x90a\t\xA6\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[\x80;\x15a\x02lW`@Qc\x99&\xEE}`\xE0\x1B\x81R\x91_\x91\x83\x91\x82\x90\x84\x90\x82\x90a\x10v\x903`\x04\x84\x01a\x18\xD9V[\x03\x92Z\xF1\x80\x15a\n2Wa\x10\xB9W[P`hT`\x01`\x01`\xA0\x1B\x03\x163\x7F\xA4S\xDBa*\xF5\x9EU!\xD6\xAB\x92\x84\xDC>-\x06\xAF(n\xB1\xB1\xB7\xB7q\xFC\xE4ql\x19\xF2\xC1_\x80\xA3V[\x80a\n&_a\x10\xC7\x93a\x02\x12V[_a\x10\x85V[cB\xEEh\xB5`\xE0\x1B_R`\x04_\xFD[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[\x80Q\x82\x10\x15a\x11\x04W` \x91`\x05\x1B\x01\x01\x90V[a\x10\xDCV[\x80Q\x15a\x11\x04W` \x01Q\x80Q`eT\x03a\x11'Wa\x02B\x90a\x14dV[c\x16\x9E\xFB[`\xE1\x1B_R`\x04_\xFD[\x90a\x11@\x82a\x02DV[a\x11M`@Q\x91\x82a\x02\x12V[\x82\x81R\x80\x92a\x11^`\x1F\x19\x91a\x02DV[\x01\x90` 6\x91\x017V[` \x81\x83\x03\x12a\x02lW\x80Q\x90`\x01`\x01`@\x1B\x03\x82\x11a\x02lW\x01\x90\x80`\x1F\x83\x01\x12\x15a\x02lW\x81Qa\x11\x9B\x81a\x02DV[\x92a\x11\xA9`@Q\x94\x85a\x02\x12V[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x02lW` \x01\x90[\x82\x82\x10a\x11\xD1WPPP\x90V[\x81Q\x81R` \x91\x82\x01\x91\x01a\x11\xC4V[``` \x91`@\x81\x01\x93`\x01\x80`\xA0\x1B\x03\x16\x81R`@\x83\x82\x01R\x84Q\x80\x94R\x01\x92\x01\x90_[\x81\x81\x10a\x12\x13WPPP\x90V[\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x12\x06V[`@Q=_\x82>=\x90\xFD[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x81\x81\x02\x92\x91\x81\x15\x91\x84\x04\x14\x17\x15a\r[WV[\x90`\x01\x82\x01\x80\x92\x11a\r[WV[\x91\x90\x82\x01\x80\x92\x11a\r[WV[a\x12\x87a\x0FXV[\x91_\x90a\x12\x94\x84Qa\x116V[\x91_[\x85Q\x81\x10\x15a\x12\xE0W\x80a\x12\xDAa\x12\xC1a\x12\xB3`\x01\x94\x8Aa\x10\xF0V[QQ`\x01`\x01`\xA0\x1B\x03\x16\x90V[a\x12\xCB\x83\x88a\x10\xF0V[`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90RV[\x01a\x12\x97V[P\x91_\x90a\x13\x04\x92\x94`@Q\x93\x84\x92\x83\x92c\x90\x04\x13G`\xE0\x1B\x84R`\x04\x84\x01a\x11\xE1V[\x03\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\n2W_\x91a\x13\xC4W[P_\x91[\x84Q\x83\x10\x15a\x13\xA2Wa\x13\x9A`\x01\x91a\x13\x94a\x13`\x86\x86a\x10\xF0V[Qa\x13\x8Ea\x13\x82` a\x13s\x8A\x8Da\x10\xF0V[Q\x01Q`\x01`\x01``\x1B\x03\x16\x90V[`\x01`\x01``\x1B\x03\x16\x90V[\x90a\x12QV[\x90a\x12rV[\x92\x01\x91a\x13DV[\x91PPa\x13\xB3\x91\x92Pa'\x10\x90\x04\x90V[`gT\x81\x10a\x13\xBFW\x90V[P_\x90V[a\x13\xE0\x91P=\x80_\x83>a\x13\xD8\x81\x83a\x02\x12V[\x81\x01\x90a\x11hV[_a\x13@V[\x15a\x13\xEDWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x90\xFD[\x91\x90\x91_\x83\x82\x01\x93\x84\x12\x91\x12\x90\x80\x15\x82\x16\x91\x15\x16\x17a\r[WV[\x90_\x80[\x83Q\x82\x10\x15a\x14\xA2W`\x01\x90a\x14\x9A\x90a\x14\x94`\x01`\x01`\xA0\x1B\x03a\x14\x8D\x86\x89a\x10\xF0V[Q\x16a\x1D>V[\x90a\x14IV[\x91\x01\x90a\x14hV[\x90Pa\x14\xAF\x91\x92Pa\x1EoV[PPV[_\x19\x81\x01\x91\x90\x82\x11a\r[WV[C\x81\x10\x15a\x15\xA4Wa\x14\xD2\x90a\x1E\xF4V[`kT\x90c\xFF\xFF\xFF\xFF\x16_[\x82\x81\x10a\x15?WPP\x80a\x14\xFBWP_[`\x01`\x01`\xE0\x1B\x03\x16\x90V[a\x153a\x15\na\x15:\x92a\x14\xB3V[`k_R\x7F\xBDC\xCB\x8E\xCE\x8C\xD1\x86;\xCD`\x82\xD6\\[\r%f[\x1C\xE1y\x80\xF0\xDAC\xC0\xEDT_\x98\xB4\x01\x90V[T` \x1C\x90V[a\x14\xEFV[\x90\x91\x80\x82\x16\x90\x80\x83\x18`\x01\x1C\x82\x01\x80\x92\x11a\r[W`k_R\x7F\xBDC\xCB\x8E\xCE\x8C\xD1\x86;\xCD`\x82\xD6\\[\r%f[\x1C\xE1y\x80\xF0\xDAC\xC0\xEDT_\x98\xB4\x82\x01Tc\xFF\xFF\xFF\xFF\x16\x84\x10\x15a\x15\x92WP\x91[\x90a\x14\xDEV[\x92\x91Pa\x15\x9E\x90a\x12dV[\x90a\x15\x8CV[`d`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R` `$\x82\x01R\x7FCheckpoints: block not yet mined`D\x82\x01R\xFD[C\x81\x10\x15a\x15\xA4Wa\x15\xF9\x90a\x1E\xF4V[`lT\x90c\xFF\xFF\xFF\xFF\x16_[\x82\x81\x10a\x16PWPP\x80a\x16\x18WP_\x90V[a\x153a\x16'a\x15:\x92a\x14\xB3V[`l_R\x7F+JQ\xABP_\xC9j\tR\xEF\xDA+\xA6\x1B\xCD0x\xD4\xC0,9\xA1\x86\xEC\x16\xF2\x18\x83\xFB\xE0\x16\x01\x90V[\x90\x91\x80\x82\x16\x90\x80\x83\x18`\x01\x1C\x82\x01\x80\x92\x11a\r[W`l_R\x7F+JQ\xABP_\xC9j\tR\xEF\xDA+\xA6\x1B\xCD0x\xD4\xC0,9\xA1\x86\xEC\x16\xF2\x18\x83\xFB\xE0\x16\x82\x01Tc\xFF\xFF\xFF\xFF\x16\x84\x10\x15a\x16\xA3WP\x91[\x90a\x16\x05V[\x92\x91Pa\x16\xAF\x90a\x12dV[\x90a\x16\x9DV[\x90C\x81\x10\x15a\x15\xA4Wa\x16\xC7\x90a\x1E\xF4V[\x81T\x90c\xFF\xFF\xFF\xFF\x16_[\x82\x81\x10a\x17\x02WPP\x80a\x16\xE7WP_\x91\x90PV[a\x15:\x91a\x16\xF7a\x153\x92a\x14\xB3V[\x90_R` _ \x01\x90V[\x90\x91\x80\x82\x16\x90\x80\x83\x18`\x01\x1C\x82\x01\x80\x92\x11a\r[W\x84_R\x83c\xFF\xFF\xFF\xFFa\x174\x84` _ \x01c\xFF\xFF\xFF\xFF\x90T\x16\x90V[\x16\x11\x15a\x17DWP\x91[\x90a\x16\xD2V[\x92\x91Pa\x17P\x90a\x12dV[\x90a\x17>V[\x93\x92\x91\x93\x81Q\x90_\x95_\x93\x81Q\x84\x03a\x18*W\x83\x15a\x18\x1BW\x92\x91\x90_\x93[\x83\x85\x10a\x17\x8CWPPPPPa\x02B\x92\x93Pa \xE0V[\x90\x91\x92\x93\x94`\x01\x80`\xA0\x1B\x03a\x17\xA2\x87\x84a\x10\xF0V[Q\x16\x98c\xFF\xFF\xFF\xFF\x88\x16\x91C\x83\x10\x15a\x18\x0CWa\x17\xF7a\x18\x01\x92a\x17\xE5\x8D`\x01\x96\x81_R`j` Ra\x17\xDE\x88\x80`\xA0\x1B\x03\x91`@_ a\x16\xB5V[\x16\x92a\x1F\\V[\x87a\x17\xF0\x8B\x89a\x10\xF0V[Q\x91a\x1F\x80V[a\x13\x94\x89\x8Ca \xB5V[\x95\x01\x93\x92\x91\x90a\x17uV[c\xE6O\x18\x0F`\xE0\x1B_R`\x04_\xFD[c%\x1FV\xA1`\xE2\x1B_R`\x04_\xFD[c\x1F\xECgG`\xE3\x1B_R`\x04_\xFD[`kT\x80a\x18FWP_\x90V[\x80_\x19\x81\x01\x11a\r[W`k_R\x7F\xBDC\xCB\x8E\xCE\x8C\xD1\x86;\xCD`\x82\xD6\\[\r%f[\x1C\xE1y\x80\xF0\xDAC\xC0\xEDT_\x98\xB3\x01T` \x1C\x90V[\x80T\x90\x81a\x18\x8BWPP_\x90V[\x81_\x19\x81\x01\x11a\r[W_R_\x19\x90` _ \x01\x01T` \x1C\x90V[_\x19\x81\x14a\r[W`\x01\x01\x90V[\x80Q\x80\x83R` \x92\x91\x81\x90\x84\x01\x84\x84\x01^_\x82\x82\x01\x84\x01R`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[\x90`\x01\x80`\xA0\x1B\x03\x16\x81R`@` \x82\x01R`\x80`@a\x19\x04\x84Q``\x83\x86\x01R`\xA0\x85\x01\x90a\x18\xB5V[\x93` \x81\x01Q``\x85\x01R\x01Q\x91\x01R\x90V[`3T`\x01`\x01`\xA0\x1B\x03\x163\x03a\x19+WV[`d`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R` `$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R\xFD[a\x19xCa\x1E\xF4V[`\x01`\x01`\xE0\x1B\x03\x82\x11a\x19\xCCW\x7F\x93$\xF7\xE5\xA7\xC0(\x88\x08\xA64\xCC\xDED\xB8\xE9ygdt\xB2.)\xEE\x9D\xD5i\xB5^y\x1AK\x91` \x91a\x19\xC1\x90`\x01`\x01`\xE0\x1B\x03\x83\x16\x90`la\"\xA0V[PP`@Q\x90\x81R\xA1V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FSafeCast: value doesn't fit in 2`D\x82\x01Rf24 bits`\xC8\x1B`d\x82\x01R`\x84\x90\xFD[`3\x80T`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x01`\x01`\xA0\x1B\x03\x19\x82\x16\x81\x17\x90\x92U\x90\x91\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0_\x80\xA3V[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16_\x81\x81R`j` R`@\x90 \x90\x92\x91\x90a\x1A\x8E\x90a\x18}V[`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x92\x91\x16\x90\x82\x82\x14a\x1A\xF5W\x80_R`j` Ra\x1A\xB9\x83`@_ a!QV[PP`@Q`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x82RC\x91\x7F\xD0a\x16\x82R\xF4As6X\xF0\x9EM\x8F[-\x99\x8E\xD4\xEF$\xA2\xBB\xFDl\xEC\xA5.\xA11P\x02\x90` \x90\xA4V[PPPV[\x80\x15a\r[W_\x19\x01\x90V[\x15a\x1B\rWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x90\xFD[a\x1B\x9F\x92\x91a\r\xBF\x91a\x1B\x7F`\xFF_T`\x08\x1C\x16a\x1B\x06V[`\x01\x80`\xA0\x1B\x03\x16`\x01`\x01``\x1B\x03`\xA0\x1B`hT\x16\x17`hUa\x19oV[a\x1B\xB8`\xFF_T`\x08\x1C\x16a\x1B\xB3\x81a\x1B\x06V[a\x1B\x06V[a\x02B3a\x1A!V[`fT`\x01`@\x1B\x81\x10\x15a\x01\xD7W`\x01\x81\x01`fU`fT\x81\x10\x15a\x11\x04W`f_R\x81Q` \x92\x90\x92\x01Q`\xA0\x1B`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17_Q` a$v_9_Q\x90_R\x90\x91\x01UV[\x90\x91a\x1C6a\x1CD\x93`@\x84R`@\x84\x01\x90a\x04\xD3V[\x91` \x81\x84\x03\x91\x01Ra\x04\xD3V[\x90V[a\x1CP\x81a!{V[\x15a\x1D\x17W`@Q\x91a\x1Cb\x83a\x01\xBCV[a\x1Cja\x0FXV[\x83R`fT_`fU\x80a\x1C\xDDW[P_[\x82Q\x80Q\x82\x10\x15a\x1C\xA3W\x90a\x1C\x9Da\x1C\x97\x82`\x01\x94a\x10\xF0V[Qa\x1B\xC1V[\x01a\x1C|V[PP\x91\x90\x7F#\xAA\xD4\xE6\x17D\xEC\xE1d\x13\n\xA4\x15\xC1an\x80\x13k\x0F\x07p\xE5e\x89C\x8B\x90\xB2i&^\x91a\x1C\xD8`@Q\x92\x83\x92\x83a\x1C\x1FV[\x03\x90\xA1V[`f_R_Q` a$v_9_Q\x90_R\x01_Q` a$v_9_Q\x90_R[\x81\x81\x10a\x1D\x0CWPa\x1CyV[_\x81U`\x01\x01a\x1C\xFFV[c\xD1sWy`\xE0\x1B_R`\x04_\xFD[\x81\x81\x03\x92\x91_\x13\x80\x15\x82\x85\x13\x16\x91\x84\x12\x16\x17a\r[WV[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x81\x81R`m` R`@\x81 \x90\x92\x90\x83\x90a\x1Dc\x90a\x18}V[`\x01\x80`\xE0\x1B\x03\x16\x93\x84a\x1D\x92`\xFFa\x1D\x8C\x86`\x01\x80`\xA0\x1B\x03\x16_R`n` R`@_ \x90V[T\x16\x15\x90V[\x15a\x1E\x07Wa\x1D\xA0\x91a\x1D&V[\x93\x84\x15a\x1E\0Wa\x1D\xEAa\x1D\xE5\x7F\x88w\r\xC8b\xE4z~\xD5\x86\x90xW\xEB\x1Bu\xE4\xC5\xFF\xC8\xB7\x07\xC7\xEE\x10\xEBt\xD6\x88_\xE5\x94\x94`\x01\x80`\xA0\x1B\x03\x16_R`m` R`@_ \x90V[a!,V[PP[`@\x80Q\x91\x82R` \x82\x01\x92\x90\x92R\xA2\x90V[PPPP\x90V[PPPa\x1E\x13\x81a\x12\x7FV[a\x1E\x1D\x84\x82a\x1D&V[\x93\x84\x15a\x1E\0Wa\x1Eh\x82a\x1Ec\x7F\x88w\r\xC8b\xE4z~\xD5\x86\x90xW\xEB\x1Bu\xE4\xC5\xFF\xC8\xB7\x07\xC7\xEE\x10\xEBt\xD6\x88_\xE5\x94\x95`\x01\x80`\xA0\x1B\x03\x16_R`m` R`@_ \x90V[a!QV[PPa\x1D\xEDV[\x90a\x1E\x8B`\x01`\x01`\xE0\x1B\x03a\x1E\x83a\x189V[\x16\x92\x83a\x14IV[\x90a\x1E\x95Ca\x1E\xF4V[`\x01`\x01`\xE0\x1B\x03\x83\x11a\x19\xCCWa\x1E\xB9\x90`\x01`\x01`\xE0\x1B\x03\x84\x16\x90`ka\"\xA0V[PP`@\x80Q\x84\x81R` \x81\x01\x84\x90R\x7F\x86\xDC\xF8k\x12\xDF\xEE\xDE\xA7J\xE90\r\xBD\xAA\x19;\xCC\xE5\x80\x93i\xC8\x17~\xA2\xF4\xEA\xAAer\x9B\x91\x81\x90\x81\x01a\x1C\xD8V[c\xFF\xFF\xFF\xFF\x81\x11a\x1F\x08Wc\xFF\xFF\xFF\xFF\x16\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FSafeCast: value doesn't fit in 3`D\x82\x01Re2 bits`\xD0\x1B`d\x82\x01R`\x84\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x91\x16\x10\x15a\x1FqWV[c\xBAP\xF9\x11`\xE0\x1B_R`\x04_\xFD[\x91\x90\x91a\x1F\x8D\x82\x84a#\xB6V[\x93\x90\x92`\x05\x85\x10\x15a \xA1Wa\x1F\xB6\x94\x15\x93\x84a \x8BW[P\x83\x15a\x1F\xCBW[PPP\x15\x15\x15\x90V[a\x1F\xBCWV[c\x8B\xAAW\x9F`\xE0\x1B_R`\x04_\xFD[_\x93P\x90a \x03a \x11\x85\x94\x93`@Q\x92\x83\x91` \x83\x01\x95c\x0B\x13]?`\xE1\x1B\x87R`$\x84\x01R`@`D\x84\x01R`d\x83\x01\x90a\x18\xB5V[\x03`\x1F\x19\x81\x01\x83R\x82a\x02\x12V[Q\x91Z\xFA=\x15a \x84W=a %\x81a\x03TV[\x90a 3`@Q\x92\x83a\x02\x12V[\x81R=_` \x83\x01>[\x81a vW[\x81a RW[P_\x80\x80a\x1F\xADV[\x90Pa oc\x0B\x13]?`\xE1\x1B\x91` \x80\x82Q\x83\x01\x01\x91\x01a#\xEBV[\x14_a IV[\x90P` \x81Q\x10\x15\x90a CV[``a =V[`\x01`\x01`\xA0\x1B\x03\x84\x81\x16\x91\x16\x14\x93P_a\x1F\xA5V[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[\x90c\xFF\xFF\xFF\xFF\x16C\x81\x10\x15a\x18\x0CWa\x1CD\x91`\x01\x80`\xA0\x1B\x03\x16_R`m` R`@_ a\x16\xB5V[\x90c\xFF\xFF\xFF\xFF\x16C\x81\x10\x15a\x18\x0CWa \xF8\x81a\x14\xC1V[\x82\x11a!\x1DWa!\x07\x90a\x15\xE8V[\x11a!\x0EWV[c\xE1!c/`\xE0\x1B_R`\x04_\xFD[cK\x05\xA0\xF7`\xE1\x1B_R`\x04_\xFD[_a!@\x91a!:Ca\x1E\xF4V[\x90a\"\xA0V[`\x01`\x01`\xE0\x1B\x03\x91\x82\x16\x92\x91\x16\x90V[a!ZCa\x1E\xF4V[`\x01`\x01`\xE0\x1B\x03\x83\x11a\x19\xCCWa!@\x92`\x01`\x01`\xE0\x1B\x03\x16\x91a\"\xA0V[Q_\x91\x90\x82\x80[\x82Q\x82\x10\x15a!\xD1Wa!\x9Ba\t\xA6a\x12\xB3\x84\x86a\x10\xF0V[\x90`\x01`\x01`\xA0\x1B\x03\x80\x83\x16\x91\x16\x10\x15a\x1FqWa!\xC8`\x01\x91\x95a\x13\x94a\x13\x82` a\x13s\x87\x89a\x10\xF0V[\x91\x01\x90\x93a!\x82V[PPP\x90a'\x10\x14\x15_\x14a!\xE4W_\x90V[`\x01\x90V[\x80T\x90`\x01`@\x1B\x82\x10\x15a\x01\xD7W`\x01\x82\x01\x80\x82U\x82\x10\x15a\x11\x04W_\x90\x81R` \x90 \x01\x81Q` \x92\x83\x01Q\x90\x92\x1Bc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[\x90`@Qa\"?\x81a\x01\xF7V[` \x81\x93Tc\xFF\xFF\xFF\xFF\x81\x16\x83R\x81\x1C\x91\x01RV[\x15a\"[WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7FCheckpoint: decreasing keys\0\0\0\0\0`D\x82\x01R`d\x90\xFD[\x90\x92\x91\x92\x83\x82T\x80\x15\x15_\x14a#\x8CW\x92` \x92\x91\x84a\"\xD8a\"\xD3a\"\xC8a#R\x98a\x14\xB3V[\x85_R` _ \x01\x90V[a\"2V[\x93c\xFF\xFF\xFF\xFFa\"\xFDa\"\xEF\x87Qc\xFF\xFF\xFF\xFF\x16\x90V[\x82\x84\x16\x92\x83\x91\x16\x11\x15a\"TV[a#\x17a#\x0E\x87Qc\xFF\xFF\xFF\xFF\x16\x90V[c\xFF\xFF\xFF\xFF\x16\x90V[\x03a#VWPa#D\x92a\x16\xF7a#-\x92a\x14\xB3V[\x90c\xFF\xFF\xFF\xFF\x82T\x91\x81\x19\x90` \x1B\x16\x91\x16\x17\x90UV[\x01Q`\x01`\x01`\xE0\x1B\x03\x16\x90V[\x91\x90V[\x91PPa#\x87\x91a#ta#ha\x023V[c\xFF\xFF\xFF\xFF\x90\x93\x16\x83RV[`\x01`\x01`\xE0\x1B\x03\x88\x16\x82\x86\x01Ra!\xE9V[a#DV[PPa#\xB1\x91a#\x9Da#ha\x023V[`\x01`\x01`\xE0\x1B\x03\x85\x16` \x83\x01Ra!\xE9V[_\x91\x90V[\x90`A\x81Q\x14_\x14a#\xE2Wa#\xDE\x91` \x82\x01Q\x90```@\x84\x01Q\x93\x01Q_\x1A\x90a#\xFAV[\x90\x91V[PP_\x90`\x02\x90V[\x90\x81` \x91\x03\x12a\x02lWQ\x90V[\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x84\x11a$jW` \x93_\x93`\xFF`\x80\x94`@Q\x94\x85R\x16\x86\x84\x01R`@\x83\x01R``\x82\x01R\x82\x80R`\x01Z\xFA\x15a\n2W_Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a$bW\x90_\x90V[P_\x90`\x01\x90V[PPPP_\x90`\x03\x90V\xFEFP\x18y\xB8\xCA\x85%\xE8\xC2\xFDQ\x9E/\xBF\xCF\xA2\xEB\xEA&P\x12\x94\xAA\x02\xCB\xFC\xFB\x12\xE9CT\xA2dipfsX\"\x12 \x0B\xD94\xCA\x0C3\x15\x8AcN\x91\xF8\xEB\xA93`\ri\x93\xAA1\xA2f\x93R\xD7\x96\x9CT\xEC\xBB\x8EdsolcC\0\x08\x1B\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x60806040526004361015610011575f80fd5b5f3560e01c8062cf2ab5146101a35780630dba33941461019e5780631626ba7e146101995780631703a018146101945780631e4cd85e1461018f578063314f3a491461018a5780633b242e4a146101855780633d5611f61461018057806340bf2fb71461017b5780635140a548146101765780635e1042e8146101715780635ef533291461016c578063696255be14610167578063715018a614610162578063743c31f41461015d578063857dc190146101585780638da5cb5b14610153578063955f2d901461014e57806398ec1ac914610149578063a2ce5fd114610144578063ab1189951461013f578063b933fa741461013a578063dec5d1f614610135578063ec7fbb31146101305763f2fde38b1461012b575f80fd5b610e04565b610dc4565b610d60565b610cf1565b610bdf565b610ad4565b610aad565b610a5f565b610a37565b61091d565b6108d2565b610877565b6107ec565b6107c8565b610785565b6106bf565b6106a2565b610618565b6105d0565b6105a5565b610578565b61052c565b6103b5565b61031f565b6102d7565b634e487b7160e01b5f52604160045260245ffd5b602081019081106001600160401b038211176101d757604052565b6101a8565b606081019081106001600160401b038211176101d757604052565b604081019081106001600160401b038211176101d757604052565b90601f801991011681019081106001600160401b038211176101d757604052565b60405190610242604083610212565b565b6001600160401b0381116101d75760051b60200190565b6001600160a01b0381160361026c57565b5f80fd5b9080601f8301121561026c57813561028781610244565b926102956040519485610212565b81845260208085019260051b82010192831161026c57602001905b8282106102bd5750505090565b6020809183356102cc8161025b565b8152019101906102b0565b3461026c57602036600319011261026c576004356001600160401b03811161026c5761030a61030f913690600401610270565b611464565b005b63ffffffff81160361026c57565b3461026c57602036600319011261026c57602061034c63ffffffff60043561034681610311565b166114c1565b604051908152f35b6001600160401b0381116101d757601f01601f191660200190565b81601f8201121561026c5780359061038682610354565b926103946040519485610212565b8284526020838301011161026c57815f926020809301838601378301015290565b3461026c57604036600319011261026c576004356024356001600160401b03811161026c576103e890369060040161036f565b9081518201916060816020850194031261026c5760208101516001600160401b03811161026c57810183603f8201121561026c5760208101519061042b82610244565b916104396040519384610212565b8083526020808085019260051b840101019186831161026c57604001905b8282106104af5750505060408201516001600160401b03811161026c57606061048b61049896602061049294870101610e95565b9301610f4d565b92611756565b604051630b135d3f60e11b815280602081015b0390f35b6020809183516104be8161025b565b815201910190610457565b5f91031261026c57565b602060408184019251938281528451809452019201905f5b8181106104f85750505090565b825180516001600160a01b031685526020908101516001600160601b031681860152604090940193909201916001016104eb565b3461026c575f36600319011261026c57606060405161054a816101bc565b526104ab60405161055a816101bc565b610562610f58565b81526040519182916020835260208301906104d3565b3461026c57602036600319011261026c57602061034c63ffffffff60043561059f81610311565b166115e8565b3461026c575f36600319011261026c5760206001600160e01b036105c7611839565b16604051908152f35b3461026c57602036600319011261026c576004356105ed8161025b565b6001600160a01b03165f908152606d602090815260409091206001600160e01b03906105c79061187d565b3461026c57604036600319011261026c576004356001600160401b03811161026c576060600319823603011261026c57604051610654816101dc565b81600401356001600160401b03811161026c5761030f9261067d6044926004369184010161036f565b835260248101356020840152013560408201526024359061069d8261025b565b610fcf565b3461026c575f36600319011261026c576020606754604051908152f35b3461026c57604036600319011261026c576004356001600160401b03811161026c573660238201121561026c5780600401356106fa81610244565b916107086040519384610212565b8183526024602084019260051b8201019036821161026c5760248101925b82841061075657602435856001600160401b03821161026c5761075061030f92369060040161036f565b50611109565b83356001600160401b03811161026c5760209161077a839260243691870101610270565b815201930192610726565b3461026c57604036600319011261026c5760206004356107a48161025b565b6001600160a01b039081165f908152606a8352604090206105c790602435906116b5565b3461026c57602036600319011261026c5761030f6004356107e7611917565b61196f565b3461026c57604036600319011261026c576024356004356001600160401b03821161026c577f713ca53b88d6eb63f5b1854cb8cbdd736ec51eda225e46791aa9298b0160648f61084361030f933690600401610270565b9161084c611917565b6067548160675561086f6040519283928360209093929193604081019481520152565b0390a1611464565b3461026c575f36600319011261026c5761088f611917565b603380546001600160a01b031981169091555f906001600160a01b03167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e08280a3005b3461026c57602036600319011261026c576004356108ef8161025b565b335f52606e60205260ff60405f2054161561090e5761030f9033611a69565b6325ec6c1f60e01b5f5260045ffd5b3461026c575f36600319011261026c57335f908152606e6020526040902061094f9061094b905b5460ff1690565b1590565b61090e57610966610961606554611afa565b606555565b335f908152606e60205260409020805460ff1916905561098d61098833611d3e565b611e6f565b50506068546109b2906109a6906001600160a01b031681565b6001600160a01b031690565b803b1561026c576040516351b27a6d60e11b8152336004820152905f908290602490829084905af18015610a3257610a18575b6068546001600160a01b0316337f31e0adfec71bccee37b6e83a90c2fedb17d8f1693fee863c4771e7bfe2aed5805f80a3005b80610a265f610a2c93610212565b806104c9565b5f6109e5565b611232565b3461026c575f36600319011261026c576033546040516001600160a01b039091168152602090f35b3461026c57604036600319011261026c57602061034c600435610a818161025b565b60243590610a8e82610311565b60018060a01b03165f52606d835263ffffffff60405f209116906116b5565b3461026c57602036600319011261026c57602061034c600435610acf8161025b565b61127f565b3461026c57602036600319011261026c57600435610af18161025b565b60018060a01b03165f52606a602052602060018060a01b036105c760405f2061187d565b919060208382031261026c57604051610b2d816101bc565b80938035906001600160401b03821161026c570182601f8201121561026c57803590610b5882610244565b93610b666040519586610212565b82855260208086019360061b8301019181831161026c57602001925b828410610b90575050505052565b60408483031261026c5760405190610ba7826101f7565b8435610bb28161025b565b82526020850135906001600160601b038216820361026c5782602092836040950152815201930192610b82565b3461026c57606036600319011261026c57600435610bfc8161025b565b6044356024356001600160401b03821161026c57610c21610c69923690600401610b15565b905f5493610c4f610c3961094b8760ff9060081c1690565b80968197610ce3575b8115610cc3575b506113e6565b84610c60600160ff195f5416175f55565b610cac57611b66565b610c6f57005b610c7d61ff00195f54165f55565b604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb384740249890602090a1005b610cbe61010061ff00195f5416175f55565b611b66565b303b15915081610cd5575b505f610c49565b60ff1660011490505f610cce565b600160ff8216109150610c42565b3461026c575f36600319011261026c57606c5480610d20575060205f5b6040516001600160e01b039091168152f35b805f19810111610d5b57606c5f527f2b4a51ab505fc96a0952efda2ba61bcd3078d4c02c39a186ec16f21883fbe0150154602090811c610d0e565b61123d565b3461026c57604036600319011261026c576004356001600160401b03811161026c57610d90903690600401610b15565b602435906001600160401b03821161026c5761030a610db661030f933690600401610270565b91610dbf611917565b611c47565b3461026c57602036600319011261026c57600435610de18161025b565b60018060a01b03165f52606e602052602060ff60405f2054166040519015158152f35b3461026c57602036600319011261026c57600435610e218161025b565b610e29611917565b6001600160a01b03811615610e415761030f90611a21565b60405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b6064820152608490fd5b9080601f8301121561026c57815191610ead83610244565b92610ebb6040519485610212565b80845260208085019160051b8301019183831161026c5760208101915b838310610ee757505050505090565b82516001600160401b03811161026c57820185603f8201121561026c57602081015191610f1383610354565b610f206040519182610212565b838152604083850101881061026c575f602085819660408397018386015e83010152815201920191610ed8565b519061024282610311565b60665490610f6582610244565b91610f736040519384610212565b80835260665f9081525f5160206124765f395f51905f52602085015b838310610f9c5750505050565b600160208192604051610fae816101f7565b8554848060a01b038116825260a01c83820152815201920192019190610f8f565b335f908152606e60205260409020909190610fe990610944565b6110cd5761103290610fff6109616065546118a7565b335f908152606e6020526040902061101e90805460ff19166001179055565b61102a61098833611d3e565b505033611a69565b606854611049906109a6906001600160a01b031681565b803b1561026c57604051639926ee7d60e01b8152915f9183918290849082906110769033600484016118d9565b03925af18015610a32576110b9575b506068546001600160a01b0316337fa453db612af59e5521d6ab9284dc3e2d06af286eb1b1b7b771fce4716c19f2c15f80a3565b80610a265f6110c793610212565b5f611085565b6342ee68b560e01b5f5260045ffd5b634e487b7160e01b5f52603260045260245ffd5b80518210156111045760209160051b010190565b6110dc565b80511561110457602001518051606554036111275761024290611464565b63169efb5b60e11b5f5260045ffd5b9061114082610244565b61114d6040519182610212565b828152809261115e601f1991610244565b0190602036910137565b60208183031261026c578051906001600160401b03821161026c57019080601f8301121561026c57815161119b81610244565b926111a96040519485610212565b81845260208085019260051b82010192831161026c57602001905b8282106111d15750505090565b81518152602091820191016111c4565b6060602091604081019360018060a01b031681526040838201528451809452019201905f5b8181106112135750505090565b82516001600160a01b0316845260209384019390920191600101611206565b6040513d5f823e3d90fd5b634e487b7160e01b5f52601160045260245ffd5b81810292918115918404141715610d5b57565b9060018201809211610d5b57565b91908201809211610d5b57565b611287610f58565b915f906112948451611136565b915f5b85518110156112e057806112da6112c16112b36001948a6110f0565b51516001600160a01b031690565b6112cb83886110f0565b6001600160a01b039091169052565b01611297565b50915f9061130492946040519384928392639004134760e01b8452600484016111e1565b03817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa908115610a32575f916113c4575b505f915b84518310156113a25761139a60019161139461136086866110f0565b5161138e61138260206113738a8d6110f0565b5101516001600160601b031690565b6001600160601b031690565b90611251565b90611272565b920191611344565b9150506113b3919250612710900490565b60675481106113bf5790565b505f90565b6113e091503d805f833e6113d88183610212565b810190611168565b5f611340565b156113ed57565b60405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b6064820152608490fd5b9190915f8382019384129112908015821691151617610d5b57565b905f805b83518210156114a25760019061149a906114946001600160a01b0361148d86896110f0565b5116611d3e565b90611449565b910190611468565b90506114af919250611e6f565b5050565b5f19810191908211610d5b57565b438110156115a4576114d290611ef4565b606b549063ffffffff165f5b82811061153f575050806114fb57505f5b6001600160e01b031690565b61153361150a61153a926114b3565b606b5f527fbd43cb8ece8cd1863bcd6082d65c5b0d25665b1ce17980f0da43c0ed545f98b40190565b5460201c90565b6114ef565b90918082169080831860011c8201809211610d5b57606b5f527fbd43cb8ece8cd1863bcd6082d65c5b0d25665b1ce17980f0da43c0ed545f98b482015463ffffffff168410156115925750915b906114de565b92915061159e90611264565b9061158c565b606460405162461bcd60e51b815260206004820152602060248201527f436865636b706f696e74733a20626c6f636b206e6f7420796574206d696e65646044820152fd5b438110156115a4576115f990611ef4565b606c549063ffffffff165f5b8281106116505750508061161857505f90565b61153361162761153a926114b3565b606c5f527f2b4a51ab505fc96a0952efda2ba61bcd3078d4c02c39a186ec16f21883fbe0160190565b90918082169080831860011c8201809211610d5b57606c5f527f2b4a51ab505fc96a0952efda2ba61bcd3078d4c02c39a186ec16f21883fbe01682015463ffffffff168410156116a35750915b90611605565b9291506116af90611264565b9061169d565b90438110156115a4576116c790611ef4565b81549063ffffffff165f5b828110611702575050806116e757505f919050565b61153a916116f7611533926114b3565b905f5260205f200190565b90918082169080831860011c8201809211610d5b57845f528363ffffffff6117348460205f200163ffffffff90541690565b1611156117445750915b906116d2565b92915061175090611264565b9061173e565b939291938151905f955f938151840361182a57831561181b579291905f935b83851061178c5750505050506102429293506120e0565b909192939460018060a01b036117a287846110f0565b51169863ffffffff8816914383101561180c576117f7611801926117e58d600196815f52606a6020526117de888060a01b039160405f206116b5565b1692611f5c565b876117f08b896110f0565b5191611f80565b611394898c6120b5565b950193929190611775565b63e64f180f60e01b5f5260045ffd5b63251f56a160e21b5f5260045ffd5b631fec674760e31b5f5260045ffd5b606b548061184657505f90565b805f19810111610d5b57606b5f527fbd43cb8ece8cd1863bcd6082d65c5b0d25665b1ce17980f0da43c0ed545f98b3015460201c90565b8054908161188b5750505f90565b815f19810111610d5b575f525f199060205f2001015460201c90565b5f198114610d5b5760010190565b805180835260209291819084018484015e5f828201840152601f01601f1916010190565b9060018060a01b031681526040602082015260806040611904845160608386015260a08501906118b5565b9360208101516060850152015191015290565b6033546001600160a01b0316330361192b57565b606460405162461bcd60e51b815260206004820152602060248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152fd5b61197843611ef4565b6001600160e01b0382116119cc577f9324f7e5a7c0288808a634ccde44b8e979676474b22e29ee9dd569b55e791a4b916020916119c1906001600160e01b03831690606c6122a0565b5050604051908152a1565b60405162461bcd60e51b815260206004820152602760248201527f53616665436173743a2076616c756520646f65736e27742066697420696e20326044820152663234206269747360c81b6064820152608490fd5b603380546001600160a01b039283166001600160a01b0319821681179092559091167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e05f80a3565b6001600160a01b039081165f818152606a6020526040902090929190611a8e9061187d565b6001600160a01b0390921692911690828214611af557805f52606a602052611ab98360405f20612151565b50506040516001600160a01b03909216825243917fd061168252f441733658f09e4d8f5b2d998ed4ef24a2bbfd6ceca52ea131500290602090a4565b505050565b8015610d5b575f190190565b15611b0d57565b60405162461bcd60e51b815260206004820152602b60248201527f496e697469616c697a61626c653a20636f6e7472616374206973206e6f74206960448201526a6e697469616c697a696e6760a81b6064820152608490fd5b611b9f9291610dbf91611b7f60ff5f5460081c16611b06565b60018060a01b03166001600160601b0360a01b606854161760685561196f565b611bb860ff5f5460081c16611bb381611b06565b611b06565b61024233611a21565b606654600160401b8110156101d757600181016066556066548110156111045760665f5281516020929092015160a01b6001600160a01b0319166001600160a01b0392909216919091175f5160206124765f395f51905f5290910155565b9091611c36611c44936040845260408401906104d3565b9160208184039101526104d3565b90565b611c508161217b565b15611d175760405191611c62836101bc565b611c6a610f58565b83526066545f60665580611cdd575b505f5b82518051821015611ca35790611c9d611c97826001946110f0565b51611bc1565b01611c7c565b505091907f23aad4e61744ece164130aa415c1616e80136b0f0770e56589438b90b269265e91611cd860405192839283611c1f565b0390a1565b60665f525f5160206124765f395f51905f52015f5160206124765f395f51905f525b818110611d0c5750611c79565b5f8155600101611cff565b63d173577960e01b5f5260045ffd5b81810392915f138015828513169184121617610d5b57565b6001600160a01b0381165f818152606d602052604081209092908390611d639061187d565b60018060e01b03169384611d9260ff611d8c8660018060a01b03165f52606e60205260405f2090565b54161590565b15611e0757611da091611d26565b938415611e0057611dea611de57f88770dc862e47a7ed586907857eb1b75e4c5ffc8b707c7ee10eb74d6885fe5949460018060a01b03165f52606d60205260405f2090565b61212c565b50505b604080519182526020820192909252a290565b5050505090565b505050611e138161127f565b611e1d8482611d26565b938415611e0057611e6882611e637f88770dc862e47a7ed586907857eb1b75e4c5ffc8b707c7ee10eb74d6885fe5949560018060a01b03165f52606d60205260405f2090565b612151565b5050611ded565b90611e8b6001600160e01b03611e83611839565b169283611449565b90611e9543611ef4565b6001600160e01b0383116119cc57611eb9906001600160e01b03841690606b6122a0565b505060408051848152602081018490527f86dcf86b12dfeedea74ae9300dbdaa193bcce5809369c8177ea2f4eaaa65729b9181908101611cd8565b63ffffffff8111611f085763ffffffff1690565b60405162461bcd60e51b815260206004820152602660248201527f53616665436173743a2076616c756520646f65736e27742066697420696e203360448201526532206269747360d01b6064820152608490fd5b6001600160a01b0391821691161015611f7157565b63ba50f91160e01b5f5260045ffd5b919091611f8d82846123b6565b93909260058510156120a157611fb69415938461208b575b508315611fcb575b50505015151590565b611fbc57565b638baa579f60e01b5f5260045ffd5b5f9350906120036120118594936040519283916020830195630b135d3f60e11b875260248401526040604484015260648301906118b5565b03601f198101835282610212565b51915afa3d15612084573d61202581610354565b906120336040519283610212565b81523d5f602083013e5b81612076575b81612052575b505f8080611fad565b905061206f630b135d3f60e11b91602080825183010191016123eb565b145f612049565b905060208151101590612043565b606061203d565b6001600160a01b0384811691161493505f611fa5565b634e487b7160e01b5f52602160045260245ffd5b9063ffffffff164381101561180c57611c449160018060a01b03165f52606d60205260405f206116b5565b9063ffffffff164381101561180c576120f8816114c1565b821161211d57612107906115e8565b1161210e57565b63e121632f60e01b5f5260045ffd5b634b05a0f760e11b5f5260045ffd5b5f6121409161213a43611ef4565b906122a0565b6001600160e01b0391821692911690565b61215a43611ef4565b6001600160e01b0383116119cc57612140926001600160e01b0316916122a0565b515f919082805b82518210156121d15761219b6109a66112b384866110f0565b906001600160a01b0380831691161015611f71576121c860019195611394611382602061137387896110f0565b91019093612182565b5050509061271014155f146121e4575f90565b600190565b805490600160401b8210156101d75760018201808255821015611104575f9081526020902001815160209283015190921b63ffffffff191663ffffffff92909216919091179055565b9060405161223f816101f7565b602081935463ffffffff81168352811c910152565b1561225b57565b60405162461bcd60e51b815260206004820152601b60248201527f436865636b706f696e743a2064656372656173696e67206b65797300000000006044820152606490fd5b909291928382548015155f1461238c579260209291846122d86122d36122c8612352986114b3565b855f5260205f200190565b612232565b9363ffffffff6122fd6122ef875163ffffffff1690565b828416928391161115612254565b61231761230e875163ffffffff1690565b63ffffffff1690565b036123565750612344926116f761232d926114b3565b9063ffffffff82549181199060201b169116179055565b01516001600160e01b031690565b9190565b91505061238791612374612368610233565b63ffffffff9093168352565b6001600160e01b038816828601526121e9565b612344565b50506123b19161239d612368610233565b6001600160e01b03851660208301526121e9565b5f9190565b9060418151145f146123e2576123de91602082015190606060408401519301515f1a906123fa565b9091565b50505f90600290565b9081602091031261026c575190565b7f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a0841161246a576020935f9360ff60809460405194855216868401526040830152606082015282805260015afa15610a32575f516001600160a01b0381161561246257905f90565b505f90600190565b505050505f9060039056fe46501879b8ca8525e8c2fd519e2fbfcfa2ebea26501294aa02cbfcfb12e94354a26469706673582212200bd934ca0c33158a634e91f8eba933600d6993aa31a2669352d7969c54ecbb8e64736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10\x15a\0\x11W_\x80\xFD[_5`\xE0\x1C\x80b\xCF*\xB5\x14a\x01\xA3W\x80c\r\xBA3\x94\x14a\x01\x9EW\x80c\x16&\xBA~\x14a\x01\x99W\x80c\x17\x03\xA0\x18\x14a\x01\x94W\x80c\x1EL\xD8^\x14a\x01\x8FW\x80c1O:I\x14a\x01\x8AW\x80c;$.J\x14a\x01\x85W\x80c=V\x11\xF6\x14a\x01\x80W\x80c@\xBF/\xB7\x14a\x01{W\x80cQ@\xA5H\x14a\x01vW\x80c^\x10B\xE8\x14a\x01qW\x80c^\xF53)\x14a\x01lW\x80cibU\xBE\x14a\x01gW\x80cqP\x18\xA6\x14a\x01bW\x80ct<1\xF4\x14a\x01]W\x80c\x85}\xC1\x90\x14a\x01XW\x80c\x8D\xA5\xCB[\x14a\x01SW\x80c\x95_-\x90\x14a\x01NW\x80c\x98\xEC\x1A\xC9\x14a\x01IW\x80c\xA2\xCE_\xD1\x14a\x01DW\x80c\xAB\x11\x89\x95\x14a\x01?W\x80c\xB93\xFAt\x14a\x01:W\x80c\xDE\xC5\xD1\xF6\x14a\x015W\x80c\xEC\x7F\xBB1\x14a\x010Wc\xF2\xFD\xE3\x8B\x14a\x01+W_\x80\xFD[a\x0E\x04V[a\r\xC4V[a\r`V[a\x0C\xF1V[a\x0B\xDFV[a\n\xD4V[a\n\xADV[a\n_V[a\n7V[a\t\x1DV[a\x08\xD2V[a\x08wV[a\x07\xECV[a\x07\xC8V[a\x07\x85V[a\x06\xBFV[a\x06\xA2V[a\x06\x18V[a\x05\xD0V[a\x05\xA5V[a\x05xV[a\x05,V[a\x03\xB5V[a\x03\x1FV[a\x02\xD7V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[` \x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x01\xD7W`@RV[a\x01\xA8V[``\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x01\xD7W`@RV[`@\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x01\xD7W`@RV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x01\xD7W`@RV[`@Q\x90a\x02B`@\x83a\x02\x12V[V[`\x01`\x01`@\x1B\x03\x81\x11a\x01\xD7W`\x05\x1B` \x01\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x03a\x02lWV[_\x80\xFD[\x90\x80`\x1F\x83\x01\x12\x15a\x02lW\x815a\x02\x87\x81a\x02DV[\x92a\x02\x95`@Q\x94\x85a\x02\x12V[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x02lW` \x01\x90[\x82\x82\x10a\x02\xBDWPPP\x90V[` \x80\x91\x835a\x02\xCC\x81a\x02[V[\x81R\x01\x91\x01\x90a\x02\xB0V[4a\x02lW` 6`\x03\x19\x01\x12a\x02lW`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x02lWa\x03\na\x03\x0F\x916\x90`\x04\x01a\x02pV[a\x14dV[\0[c\xFF\xFF\xFF\xFF\x81\x16\x03a\x02lWV[4a\x02lW` 6`\x03\x19\x01\x12a\x02lW` a\x03Lc\xFF\xFF\xFF\xFF`\x045a\x03F\x81a\x03\x11V[\x16a\x14\xC1V[`@Q\x90\x81R\xF3[`\x01`\x01`@\x1B\x03\x81\x11a\x01\xD7W`\x1F\x01`\x1F\x19\x16` \x01\x90V[\x81`\x1F\x82\x01\x12\x15a\x02lW\x805\x90a\x03\x86\x82a\x03TV[\x92a\x03\x94`@Q\x94\x85a\x02\x12V[\x82\x84R` \x83\x83\x01\x01\x11a\x02lW\x81_\x92` \x80\x93\x01\x83\x86\x017\x83\x01\x01R\x90V[4a\x02lW`@6`\x03\x19\x01\x12a\x02lW`\x045`$5`\x01`\x01`@\x1B\x03\x81\x11a\x02lWa\x03\xE8\x906\x90`\x04\x01a\x03oV[\x90\x81Q\x82\x01\x91``\x81` \x85\x01\x94\x03\x12a\x02lW` \x81\x01Q`\x01`\x01`@\x1B\x03\x81\x11a\x02lW\x81\x01\x83`?\x82\x01\x12\x15a\x02lW` \x81\x01Q\x90a\x04+\x82a\x02DV[\x91a\x049`@Q\x93\x84a\x02\x12V[\x80\x83R` \x80\x80\x85\x01\x92`\x05\x1B\x84\x01\x01\x01\x91\x86\x83\x11a\x02lW`@\x01\x90[\x82\x82\x10a\x04\xAFWPPP`@\x82\x01Q`\x01`\x01`@\x1B\x03\x81\x11a\x02lW``a\x04\x8Ba\x04\x98\x96` a\x04\x92\x94\x87\x01\x01a\x0E\x95V[\x93\x01a\x0FMV[\x92a\x17VV[`@Qc\x0B\x13]?`\xE1\x1B\x81R\x80` \x81\x01[\x03\x90\xF3[` \x80\x91\x83Qa\x04\xBE\x81a\x02[V[\x81R\x01\x91\x01\x90a\x04WV[_\x91\x03\x12a\x02lWV[` `@\x81\x84\x01\x92Q\x93\x82\x81R\x84Q\x80\x94R\x01\x92\x01\x90_[\x81\x81\x10a\x04\xF8WPPP\x90V[\x82Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x85R` \x90\x81\x01Q`\x01`\x01``\x1B\x03\x16\x81\x86\x01R`@\x90\x94\x01\x93\x90\x92\x01\x91`\x01\x01a\x04\xEBV[4a\x02lW_6`\x03\x19\x01\x12a\x02lW```@Qa\x05J\x81a\x01\xBCV[Ra\x04\xAB`@Qa\x05Z\x81a\x01\xBCV[a\x05ba\x0FXV[\x81R`@Q\x91\x82\x91` \x83R` \x83\x01\x90a\x04\xD3V[4a\x02lW` 6`\x03\x19\x01\x12a\x02lW` a\x03Lc\xFF\xFF\xFF\xFF`\x045a\x05\x9F\x81a\x03\x11V[\x16a\x15\xE8V[4a\x02lW_6`\x03\x19\x01\x12a\x02lW` `\x01`\x01`\xE0\x1B\x03a\x05\xC7a\x189V[\x16`@Q\x90\x81R\xF3[4a\x02lW` 6`\x03\x19\x01\x12a\x02lW`\x045a\x05\xED\x81a\x02[V[`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`m` \x90\x81R`@\x90\x91 `\x01`\x01`\xE0\x1B\x03\x90a\x05\xC7\x90a\x18}V[4a\x02lW`@6`\x03\x19\x01\x12a\x02lW`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x02lW```\x03\x19\x826\x03\x01\x12a\x02lW`@Qa\x06T\x81a\x01\xDCV[\x81`\x04\x015`\x01`\x01`@\x1B\x03\x81\x11a\x02lWa\x03\x0F\x92a\x06}`D\x92`\x046\x91\x84\x01\x01a\x03oV[\x83R`$\x81\x015` \x84\x01R\x015`@\x82\x01R`$5\x90a\x06\x9D\x82a\x02[V[a\x0F\xCFV[4a\x02lW_6`\x03\x19\x01\x12a\x02lW` `gT`@Q\x90\x81R\xF3[4a\x02lW`@6`\x03\x19\x01\x12a\x02lW`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x02lW6`#\x82\x01\x12\x15a\x02lW\x80`\x04\x015a\x06\xFA\x81a\x02DV[\x91a\x07\x08`@Q\x93\x84a\x02\x12V[\x81\x83R`$` \x84\x01\x92`\x05\x1B\x82\x01\x01\x906\x82\x11a\x02lW`$\x81\x01\x92[\x82\x84\x10a\x07VW`$5\x85`\x01`\x01`@\x1B\x03\x82\x11a\x02lWa\x07Pa\x03\x0F\x926\x90`\x04\x01a\x03oV[Pa\x11\tV[\x835`\x01`\x01`@\x1B\x03\x81\x11a\x02lW` \x91a\x07z\x83\x92`$6\x91\x87\x01\x01a\x02pV[\x81R\x01\x93\x01\x92a\x07&V[4a\x02lW`@6`\x03\x19\x01\x12a\x02lW` `\x045a\x07\xA4\x81a\x02[V[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16_\x90\x81R`j\x83R`@\x90 a\x05\xC7\x90`$5\x90a\x16\xB5V[4a\x02lW` 6`\x03\x19\x01\x12a\x02lWa\x03\x0F`\x045a\x07\xE7a\x19\x17V[a\x19oV[4a\x02lW`@6`\x03\x19\x01\x12a\x02lW`$5`\x045`\x01`\x01`@\x1B\x03\x82\x11a\x02lW\x7Fq<\xA5;\x88\xD6\xEBc\xF5\xB1\x85L\xB8\xCB\xDDsn\xC5\x1E\xDA\"^Fy\x1A\xA9)\x8B\x01`d\x8Fa\x08Ca\x03\x0F\x936\x90`\x04\x01a\x02pV[\x91a\x08La\x19\x17V[`gT\x81`gUa\x08o`@Q\x92\x83\x92\x83` \x90\x93\x92\x91\x93`@\x81\x01\x94\x81R\x01RV[\x03\x90\xA1a\x14dV[4a\x02lW_6`\x03\x19\x01\x12a\x02lWa\x08\x8Fa\x19\x17V[`3\x80T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16\x90\x91U_\x90`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x82\x80\xA3\0[4a\x02lW` 6`\x03\x19\x01\x12a\x02lW`\x045a\x08\xEF\x81a\x02[V[3_R`n` R`\xFF`@_ T\x16\x15a\t\x0EWa\x03\x0F\x903a\x1AiV[c%\xECl\x1F`\xE0\x1B_R`\x04_\xFD[4a\x02lW_6`\x03\x19\x01\x12a\x02lW3_\x90\x81R`n` R`@\x90 a\tO\x90a\tK\x90[T`\xFF\x16\x90V[\x15\x90V[a\t\x0EWa\tfa\ta`eTa\x1A\xFAV[`eUV[3_\x90\x81R`n` R`@\x90 \x80T`\xFF\x19\x16\x90Ua\t\x8Da\t\x883a\x1D>V[a\x1EoV[PP`hTa\t\xB2\x90a\t\xA6\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x80;\x15a\x02lW`@QcQ\xB2zm`\xE1\x1B\x81R3`\x04\x82\x01R\x90_\x90\x82\x90`$\x90\x82\x90\x84\x90Z\xF1\x80\x15a\n2Wa\n\x18W[`hT`\x01`\x01`\xA0\x1B\x03\x163\x7F1\xE0\xAD\xFE\xC7\x1B\xCC\xEE7\xB6\xE8:\x90\xC2\xFE\xDB\x17\xD8\xF1i?\xEE\x86<Gq\xE7\xBF\xE2\xAE\xD5\x80_\x80\xA3\0[\x80a\n&_a\n,\x93a\x02\x12V[\x80a\x04\xC9V[_a\t\xE5V[a\x122V[4a\x02lW_6`\x03\x19\x01\x12a\x02lW`3T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x02lW`@6`\x03\x19\x01\x12a\x02lW` a\x03L`\x045a\n\x81\x81a\x02[V[`$5\x90a\n\x8E\x82a\x03\x11V[`\x01\x80`\xA0\x1B\x03\x16_R`m\x83Rc\xFF\xFF\xFF\xFF`@_ \x91\x16\x90a\x16\xB5V[4a\x02lW` 6`\x03\x19\x01\x12a\x02lW` a\x03L`\x045a\n\xCF\x81a\x02[V[a\x12\x7FV[4a\x02lW` 6`\x03\x19\x01\x12a\x02lW`\x045a\n\xF1\x81a\x02[V[`\x01\x80`\xA0\x1B\x03\x16_R`j` R` `\x01\x80`\xA0\x1B\x03a\x05\xC7`@_ a\x18}V[\x91\x90` \x83\x82\x03\x12a\x02lW`@Qa\x0B-\x81a\x01\xBCV[\x80\x93\x805\x90`\x01`\x01`@\x1B\x03\x82\x11a\x02lW\x01\x82`\x1F\x82\x01\x12\x15a\x02lW\x805\x90a\x0BX\x82a\x02DV[\x93a\x0Bf`@Q\x95\x86a\x02\x12V[\x82\x85R` \x80\x86\x01\x93`\x06\x1B\x83\x01\x01\x91\x81\x83\x11a\x02lW` \x01\x92[\x82\x84\x10a\x0B\x90WPPPPRV[`@\x84\x83\x03\x12a\x02lW`@Q\x90a\x0B\xA7\x82a\x01\xF7V[\x845a\x0B\xB2\x81a\x02[V[\x82R` \x85\x015\x90`\x01`\x01``\x1B\x03\x82\x16\x82\x03a\x02lW\x82` \x92\x83`@\x95\x01R\x81R\x01\x93\x01\x92a\x0B\x82V[4a\x02lW``6`\x03\x19\x01\x12a\x02lW`\x045a\x0B\xFC\x81a\x02[V[`D5`$5`\x01`\x01`@\x1B\x03\x82\x11a\x02lWa\x0C!a\x0Ci\x926\x90`\x04\x01a\x0B\x15V[\x90_T\x93a\x0COa\x0C9a\tK\x87`\xFF\x90`\x08\x1C\x16\x90V[\x80\x96\x81\x97a\x0C\xE3W[\x81\x15a\x0C\xC3W[Pa\x13\xE6V[\x84a\x0C``\x01`\xFF\x19_T\x16\x17_UV[a\x0C\xACWa\x1BfV[a\x0CoW\0[a\x0C}a\xFF\0\x19_T\x16_UV[`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x90\xA1\0[a\x0C\xBEa\x01\0a\xFF\0\x19_T\x16\x17_UV[a\x1BfV[0;\x15\x91P\x81a\x0C\xD5W[P_a\x0CIV[`\xFF\x16`\x01\x14\x90P_a\x0C\xCEV[`\x01`\xFF\x82\x16\x10\x91Pa\x0CBV[4a\x02lW_6`\x03\x19\x01\x12a\x02lW`lT\x80a\r WP` _[`@Q`\x01`\x01`\xE0\x1B\x03\x90\x91\x16\x81R\xF3[\x80_\x19\x81\x01\x11a\r[W`l_R\x7F+JQ\xABP_\xC9j\tR\xEF\xDA+\xA6\x1B\xCD0x\xD4\xC0,9\xA1\x86\xEC\x16\xF2\x18\x83\xFB\xE0\x15\x01T` \x90\x81\x1Ca\r\x0EV[a\x12=V[4a\x02lW`@6`\x03\x19\x01\x12a\x02lW`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x02lWa\r\x90\x906\x90`\x04\x01a\x0B\x15V[`$5\x90`\x01`\x01`@\x1B\x03\x82\x11a\x02lWa\x03\na\r\xB6a\x03\x0F\x936\x90`\x04\x01a\x02pV[\x91a\r\xBFa\x19\x17V[a\x1CGV[4a\x02lW` 6`\x03\x19\x01\x12a\x02lW`\x045a\r\xE1\x81a\x02[V[`\x01\x80`\xA0\x1B\x03\x16_R`n` R` `\xFF`@_ T\x16`@Q\x90\x15\x15\x81R\xF3[4a\x02lW` 6`\x03\x19\x01\x12a\x02lW`\x045a\x0E!\x81a\x02[V[a\x0E)a\x19\x17V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a\x0EAWa\x03\x0F\x90a\x1A!V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x90\xFD[\x90\x80`\x1F\x83\x01\x12\x15a\x02lW\x81Q\x91a\x0E\xAD\x83a\x02DV[\x92a\x0E\xBB`@Q\x94\x85a\x02\x12V[\x80\x84R` \x80\x85\x01\x91`\x05\x1B\x83\x01\x01\x91\x83\x83\x11a\x02lW` \x81\x01\x91[\x83\x83\x10a\x0E\xE7WPPPPP\x90V[\x82Q`\x01`\x01`@\x1B\x03\x81\x11a\x02lW\x82\x01\x85`?\x82\x01\x12\x15a\x02lW` \x81\x01Q\x91a\x0F\x13\x83a\x03TV[a\x0F `@Q\x91\x82a\x02\x12V[\x83\x81R`@\x83\x85\x01\x01\x88\x10a\x02lW_` \x85\x81\x96`@\x83\x97\x01\x83\x86\x01^\x83\x01\x01R\x81R\x01\x92\x01\x91a\x0E\xD8V[Q\x90a\x02B\x82a\x03\x11V[`fT\x90a\x0Fe\x82a\x02DV[\x91a\x0Fs`@Q\x93\x84a\x02\x12V[\x80\x83R`f_\x90\x81R_Q` a$v_9_Q\x90_R` \x85\x01[\x83\x83\x10a\x0F\x9CWPPPPV[`\x01` \x81\x92`@Qa\x0F\xAE\x81a\x01\xF7V[\x85T\x84\x80`\xA0\x1B\x03\x81\x16\x82R`\xA0\x1C\x83\x82\x01R\x81R\x01\x92\x01\x92\x01\x91\x90a\x0F\x8FV[3_\x90\x81R`n` R`@\x90 \x90\x91\x90a\x0F\xE9\x90a\tDV[a\x10\xCDWa\x102\x90a\x0F\xFFa\ta`eTa\x18\xA7V[3_\x90\x81R`n` R`@\x90 a\x10\x1E\x90\x80T`\xFF\x19\x16`\x01\x17\x90UV[a\x10*a\t\x883a\x1D>V[PP3a\x1AiV[`hTa\x10I\x90a\t\xA6\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[\x80;\x15a\x02lW`@Qc\x99&\xEE}`\xE0\x1B\x81R\x91_\x91\x83\x91\x82\x90\x84\x90\x82\x90a\x10v\x903`\x04\x84\x01a\x18\xD9V[\x03\x92Z\xF1\x80\x15a\n2Wa\x10\xB9W[P`hT`\x01`\x01`\xA0\x1B\x03\x163\x7F\xA4S\xDBa*\xF5\x9EU!\xD6\xAB\x92\x84\xDC>-\x06\xAF(n\xB1\xB1\xB7\xB7q\xFC\xE4ql\x19\xF2\xC1_\x80\xA3V[\x80a\n&_a\x10\xC7\x93a\x02\x12V[_a\x10\x85V[cB\xEEh\xB5`\xE0\x1B_R`\x04_\xFD[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[\x80Q\x82\x10\x15a\x11\x04W` \x91`\x05\x1B\x01\x01\x90V[a\x10\xDCV[\x80Q\x15a\x11\x04W` \x01Q\x80Q`eT\x03a\x11'Wa\x02B\x90a\x14dV[c\x16\x9E\xFB[`\xE1\x1B_R`\x04_\xFD[\x90a\x11@\x82a\x02DV[a\x11M`@Q\x91\x82a\x02\x12V[\x82\x81R\x80\x92a\x11^`\x1F\x19\x91a\x02DV[\x01\x90` 6\x91\x017V[` \x81\x83\x03\x12a\x02lW\x80Q\x90`\x01`\x01`@\x1B\x03\x82\x11a\x02lW\x01\x90\x80`\x1F\x83\x01\x12\x15a\x02lW\x81Qa\x11\x9B\x81a\x02DV[\x92a\x11\xA9`@Q\x94\x85a\x02\x12V[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x02lW` \x01\x90[\x82\x82\x10a\x11\xD1WPPP\x90V[\x81Q\x81R` \x91\x82\x01\x91\x01a\x11\xC4V[``` \x91`@\x81\x01\x93`\x01\x80`\xA0\x1B\x03\x16\x81R`@\x83\x82\x01R\x84Q\x80\x94R\x01\x92\x01\x90_[\x81\x81\x10a\x12\x13WPPP\x90V[\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x12\x06V[`@Q=_\x82>=\x90\xFD[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x81\x81\x02\x92\x91\x81\x15\x91\x84\x04\x14\x17\x15a\r[WV[\x90`\x01\x82\x01\x80\x92\x11a\r[WV[\x91\x90\x82\x01\x80\x92\x11a\r[WV[a\x12\x87a\x0FXV[\x91_\x90a\x12\x94\x84Qa\x116V[\x91_[\x85Q\x81\x10\x15a\x12\xE0W\x80a\x12\xDAa\x12\xC1a\x12\xB3`\x01\x94\x8Aa\x10\xF0V[QQ`\x01`\x01`\xA0\x1B\x03\x16\x90V[a\x12\xCB\x83\x88a\x10\xF0V[`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90RV[\x01a\x12\x97V[P\x91_\x90a\x13\x04\x92\x94`@Q\x93\x84\x92\x83\x92c\x90\x04\x13G`\xE0\x1B\x84R`\x04\x84\x01a\x11\xE1V[\x03\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\n2W_\x91a\x13\xC4W[P_\x91[\x84Q\x83\x10\x15a\x13\xA2Wa\x13\x9A`\x01\x91a\x13\x94a\x13`\x86\x86a\x10\xF0V[Qa\x13\x8Ea\x13\x82` a\x13s\x8A\x8Da\x10\xF0V[Q\x01Q`\x01`\x01``\x1B\x03\x16\x90V[`\x01`\x01``\x1B\x03\x16\x90V[\x90a\x12QV[\x90a\x12rV[\x92\x01\x91a\x13DV[\x91PPa\x13\xB3\x91\x92Pa'\x10\x90\x04\x90V[`gT\x81\x10a\x13\xBFW\x90V[P_\x90V[a\x13\xE0\x91P=\x80_\x83>a\x13\xD8\x81\x83a\x02\x12V[\x81\x01\x90a\x11hV[_a\x13@V[\x15a\x13\xEDWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x90\xFD[\x91\x90\x91_\x83\x82\x01\x93\x84\x12\x91\x12\x90\x80\x15\x82\x16\x91\x15\x16\x17a\r[WV[\x90_\x80[\x83Q\x82\x10\x15a\x14\xA2W`\x01\x90a\x14\x9A\x90a\x14\x94`\x01`\x01`\xA0\x1B\x03a\x14\x8D\x86\x89a\x10\xF0V[Q\x16a\x1D>V[\x90a\x14IV[\x91\x01\x90a\x14hV[\x90Pa\x14\xAF\x91\x92Pa\x1EoV[PPV[_\x19\x81\x01\x91\x90\x82\x11a\r[WV[C\x81\x10\x15a\x15\xA4Wa\x14\xD2\x90a\x1E\xF4V[`kT\x90c\xFF\xFF\xFF\xFF\x16_[\x82\x81\x10a\x15?WPP\x80a\x14\xFBWP_[`\x01`\x01`\xE0\x1B\x03\x16\x90V[a\x153a\x15\na\x15:\x92a\x14\xB3V[`k_R\x7F\xBDC\xCB\x8E\xCE\x8C\xD1\x86;\xCD`\x82\xD6\\[\r%f[\x1C\xE1y\x80\xF0\xDAC\xC0\xEDT_\x98\xB4\x01\x90V[T` \x1C\x90V[a\x14\xEFV[\x90\x91\x80\x82\x16\x90\x80\x83\x18`\x01\x1C\x82\x01\x80\x92\x11a\r[W`k_R\x7F\xBDC\xCB\x8E\xCE\x8C\xD1\x86;\xCD`\x82\xD6\\[\r%f[\x1C\xE1y\x80\xF0\xDAC\xC0\xEDT_\x98\xB4\x82\x01Tc\xFF\xFF\xFF\xFF\x16\x84\x10\x15a\x15\x92WP\x91[\x90a\x14\xDEV[\x92\x91Pa\x15\x9E\x90a\x12dV[\x90a\x15\x8CV[`d`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R` `$\x82\x01R\x7FCheckpoints: block not yet mined`D\x82\x01R\xFD[C\x81\x10\x15a\x15\xA4Wa\x15\xF9\x90a\x1E\xF4V[`lT\x90c\xFF\xFF\xFF\xFF\x16_[\x82\x81\x10a\x16PWPP\x80a\x16\x18WP_\x90V[a\x153a\x16'a\x15:\x92a\x14\xB3V[`l_R\x7F+JQ\xABP_\xC9j\tR\xEF\xDA+\xA6\x1B\xCD0x\xD4\xC0,9\xA1\x86\xEC\x16\xF2\x18\x83\xFB\xE0\x16\x01\x90V[\x90\x91\x80\x82\x16\x90\x80\x83\x18`\x01\x1C\x82\x01\x80\x92\x11a\r[W`l_R\x7F+JQ\xABP_\xC9j\tR\xEF\xDA+\xA6\x1B\xCD0x\xD4\xC0,9\xA1\x86\xEC\x16\xF2\x18\x83\xFB\xE0\x16\x82\x01Tc\xFF\xFF\xFF\xFF\x16\x84\x10\x15a\x16\xA3WP\x91[\x90a\x16\x05V[\x92\x91Pa\x16\xAF\x90a\x12dV[\x90a\x16\x9DV[\x90C\x81\x10\x15a\x15\xA4Wa\x16\xC7\x90a\x1E\xF4V[\x81T\x90c\xFF\xFF\xFF\xFF\x16_[\x82\x81\x10a\x17\x02WPP\x80a\x16\xE7WP_\x91\x90PV[a\x15:\x91a\x16\xF7a\x153\x92a\x14\xB3V[\x90_R` _ \x01\x90V[\x90\x91\x80\x82\x16\x90\x80\x83\x18`\x01\x1C\x82\x01\x80\x92\x11a\r[W\x84_R\x83c\xFF\xFF\xFF\xFFa\x174\x84` _ \x01c\xFF\xFF\xFF\xFF\x90T\x16\x90V[\x16\x11\x15a\x17DWP\x91[\x90a\x16\xD2V[\x92\x91Pa\x17P\x90a\x12dV[\x90a\x17>V[\x93\x92\x91\x93\x81Q\x90_\x95_\x93\x81Q\x84\x03a\x18*W\x83\x15a\x18\x1BW\x92\x91\x90_\x93[\x83\x85\x10a\x17\x8CWPPPPPa\x02B\x92\x93Pa \xE0V[\x90\x91\x92\x93\x94`\x01\x80`\xA0\x1B\x03a\x17\xA2\x87\x84a\x10\xF0V[Q\x16\x98c\xFF\xFF\xFF\xFF\x88\x16\x91C\x83\x10\x15a\x18\x0CWa\x17\xF7a\x18\x01\x92a\x17\xE5\x8D`\x01\x96\x81_R`j` Ra\x17\xDE\x88\x80`\xA0\x1B\x03\x91`@_ a\x16\xB5V[\x16\x92a\x1F\\V[\x87a\x17\xF0\x8B\x89a\x10\xF0V[Q\x91a\x1F\x80V[a\x13\x94\x89\x8Ca \xB5V[\x95\x01\x93\x92\x91\x90a\x17uV[c\xE6O\x18\x0F`\xE0\x1B_R`\x04_\xFD[c%\x1FV\xA1`\xE2\x1B_R`\x04_\xFD[c\x1F\xECgG`\xE3\x1B_R`\x04_\xFD[`kT\x80a\x18FWP_\x90V[\x80_\x19\x81\x01\x11a\r[W`k_R\x7F\xBDC\xCB\x8E\xCE\x8C\xD1\x86;\xCD`\x82\xD6\\[\r%f[\x1C\xE1y\x80\xF0\xDAC\xC0\xEDT_\x98\xB3\x01T` \x1C\x90V[\x80T\x90\x81a\x18\x8BWPP_\x90V[\x81_\x19\x81\x01\x11a\r[W_R_\x19\x90` _ \x01\x01T` \x1C\x90V[_\x19\x81\x14a\r[W`\x01\x01\x90V[\x80Q\x80\x83R` \x92\x91\x81\x90\x84\x01\x84\x84\x01^_\x82\x82\x01\x84\x01R`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[\x90`\x01\x80`\xA0\x1B\x03\x16\x81R`@` \x82\x01R`\x80`@a\x19\x04\x84Q``\x83\x86\x01R`\xA0\x85\x01\x90a\x18\xB5V[\x93` \x81\x01Q``\x85\x01R\x01Q\x91\x01R\x90V[`3T`\x01`\x01`\xA0\x1B\x03\x163\x03a\x19+WV[`d`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R` `$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R\xFD[a\x19xCa\x1E\xF4V[`\x01`\x01`\xE0\x1B\x03\x82\x11a\x19\xCCW\x7F\x93$\xF7\xE5\xA7\xC0(\x88\x08\xA64\xCC\xDED\xB8\xE9ygdt\xB2.)\xEE\x9D\xD5i\xB5^y\x1AK\x91` \x91a\x19\xC1\x90`\x01`\x01`\xE0\x1B\x03\x83\x16\x90`la\"\xA0V[PP`@Q\x90\x81R\xA1V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FSafeCast: value doesn't fit in 2`D\x82\x01Rf24 bits`\xC8\x1B`d\x82\x01R`\x84\x90\xFD[`3\x80T`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x01`\x01`\xA0\x1B\x03\x19\x82\x16\x81\x17\x90\x92U\x90\x91\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0_\x80\xA3V[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16_\x81\x81R`j` R`@\x90 \x90\x92\x91\x90a\x1A\x8E\x90a\x18}V[`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x92\x91\x16\x90\x82\x82\x14a\x1A\xF5W\x80_R`j` Ra\x1A\xB9\x83`@_ a!QV[PP`@Q`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x82RC\x91\x7F\xD0a\x16\x82R\xF4As6X\xF0\x9EM\x8F[-\x99\x8E\xD4\xEF$\xA2\xBB\xFDl\xEC\xA5.\xA11P\x02\x90` \x90\xA4V[PPPV[\x80\x15a\r[W_\x19\x01\x90V[\x15a\x1B\rWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x90\xFD[a\x1B\x9F\x92\x91a\r\xBF\x91a\x1B\x7F`\xFF_T`\x08\x1C\x16a\x1B\x06V[`\x01\x80`\xA0\x1B\x03\x16`\x01`\x01``\x1B\x03`\xA0\x1B`hT\x16\x17`hUa\x19oV[a\x1B\xB8`\xFF_T`\x08\x1C\x16a\x1B\xB3\x81a\x1B\x06V[a\x1B\x06V[a\x02B3a\x1A!V[`fT`\x01`@\x1B\x81\x10\x15a\x01\xD7W`\x01\x81\x01`fU`fT\x81\x10\x15a\x11\x04W`f_R\x81Q` \x92\x90\x92\x01Q`\xA0\x1B`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17_Q` a$v_9_Q\x90_R\x90\x91\x01UV[\x90\x91a\x1C6a\x1CD\x93`@\x84R`@\x84\x01\x90a\x04\xD3V[\x91` \x81\x84\x03\x91\x01Ra\x04\xD3V[\x90V[a\x1CP\x81a!{V[\x15a\x1D\x17W`@Q\x91a\x1Cb\x83a\x01\xBCV[a\x1Cja\x0FXV[\x83R`fT_`fU\x80a\x1C\xDDW[P_[\x82Q\x80Q\x82\x10\x15a\x1C\xA3W\x90a\x1C\x9Da\x1C\x97\x82`\x01\x94a\x10\xF0V[Qa\x1B\xC1V[\x01a\x1C|V[PP\x91\x90\x7F#\xAA\xD4\xE6\x17D\xEC\xE1d\x13\n\xA4\x15\xC1an\x80\x13k\x0F\x07p\xE5e\x89C\x8B\x90\xB2i&^\x91a\x1C\xD8`@Q\x92\x83\x92\x83a\x1C\x1FV[\x03\x90\xA1V[`f_R_Q` a$v_9_Q\x90_R\x01_Q` a$v_9_Q\x90_R[\x81\x81\x10a\x1D\x0CWPa\x1CyV[_\x81U`\x01\x01a\x1C\xFFV[c\xD1sWy`\xE0\x1B_R`\x04_\xFD[\x81\x81\x03\x92\x91_\x13\x80\x15\x82\x85\x13\x16\x91\x84\x12\x16\x17a\r[WV[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x81\x81R`m` R`@\x81 \x90\x92\x90\x83\x90a\x1Dc\x90a\x18}V[`\x01\x80`\xE0\x1B\x03\x16\x93\x84a\x1D\x92`\xFFa\x1D\x8C\x86`\x01\x80`\xA0\x1B\x03\x16_R`n` R`@_ \x90V[T\x16\x15\x90V[\x15a\x1E\x07Wa\x1D\xA0\x91a\x1D&V[\x93\x84\x15a\x1E\0Wa\x1D\xEAa\x1D\xE5\x7F\x88w\r\xC8b\xE4z~\xD5\x86\x90xW\xEB\x1Bu\xE4\xC5\xFF\xC8\xB7\x07\xC7\xEE\x10\xEBt\xD6\x88_\xE5\x94\x94`\x01\x80`\xA0\x1B\x03\x16_R`m` R`@_ \x90V[a!,V[PP[`@\x80Q\x91\x82R` \x82\x01\x92\x90\x92R\xA2\x90V[PPPP\x90V[PPPa\x1E\x13\x81a\x12\x7FV[a\x1E\x1D\x84\x82a\x1D&V[\x93\x84\x15a\x1E\0Wa\x1Eh\x82a\x1Ec\x7F\x88w\r\xC8b\xE4z~\xD5\x86\x90xW\xEB\x1Bu\xE4\xC5\xFF\xC8\xB7\x07\xC7\xEE\x10\xEBt\xD6\x88_\xE5\x94\x95`\x01\x80`\xA0\x1B\x03\x16_R`m` R`@_ \x90V[a!QV[PPa\x1D\xEDV[\x90a\x1E\x8B`\x01`\x01`\xE0\x1B\x03a\x1E\x83a\x189V[\x16\x92\x83a\x14IV[\x90a\x1E\x95Ca\x1E\xF4V[`\x01`\x01`\xE0\x1B\x03\x83\x11a\x19\xCCWa\x1E\xB9\x90`\x01`\x01`\xE0\x1B\x03\x84\x16\x90`ka\"\xA0V[PP`@\x80Q\x84\x81R` \x81\x01\x84\x90R\x7F\x86\xDC\xF8k\x12\xDF\xEE\xDE\xA7J\xE90\r\xBD\xAA\x19;\xCC\xE5\x80\x93i\xC8\x17~\xA2\xF4\xEA\xAAer\x9B\x91\x81\x90\x81\x01a\x1C\xD8V[c\xFF\xFF\xFF\xFF\x81\x11a\x1F\x08Wc\xFF\xFF\xFF\xFF\x16\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FSafeCast: value doesn't fit in 3`D\x82\x01Re2 bits`\xD0\x1B`d\x82\x01R`\x84\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x91\x16\x10\x15a\x1FqWV[c\xBAP\xF9\x11`\xE0\x1B_R`\x04_\xFD[\x91\x90\x91a\x1F\x8D\x82\x84a#\xB6V[\x93\x90\x92`\x05\x85\x10\x15a \xA1Wa\x1F\xB6\x94\x15\x93\x84a \x8BW[P\x83\x15a\x1F\xCBW[PPP\x15\x15\x15\x90V[a\x1F\xBCWV[c\x8B\xAAW\x9F`\xE0\x1B_R`\x04_\xFD[_\x93P\x90a \x03a \x11\x85\x94\x93`@Q\x92\x83\x91` \x83\x01\x95c\x0B\x13]?`\xE1\x1B\x87R`$\x84\x01R`@`D\x84\x01R`d\x83\x01\x90a\x18\xB5V[\x03`\x1F\x19\x81\x01\x83R\x82a\x02\x12V[Q\x91Z\xFA=\x15a \x84W=a %\x81a\x03TV[\x90a 3`@Q\x92\x83a\x02\x12V[\x81R=_` \x83\x01>[\x81a vW[\x81a RW[P_\x80\x80a\x1F\xADV[\x90Pa oc\x0B\x13]?`\xE1\x1B\x91` \x80\x82Q\x83\x01\x01\x91\x01a#\xEBV[\x14_a IV[\x90P` \x81Q\x10\x15\x90a CV[``a =V[`\x01`\x01`\xA0\x1B\x03\x84\x81\x16\x91\x16\x14\x93P_a\x1F\xA5V[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[\x90c\xFF\xFF\xFF\xFF\x16C\x81\x10\x15a\x18\x0CWa\x1CD\x91`\x01\x80`\xA0\x1B\x03\x16_R`m` R`@_ a\x16\xB5V[\x90c\xFF\xFF\xFF\xFF\x16C\x81\x10\x15a\x18\x0CWa \xF8\x81a\x14\xC1V[\x82\x11a!\x1DWa!\x07\x90a\x15\xE8V[\x11a!\x0EWV[c\xE1!c/`\xE0\x1B_R`\x04_\xFD[cK\x05\xA0\xF7`\xE1\x1B_R`\x04_\xFD[_a!@\x91a!:Ca\x1E\xF4V[\x90a\"\xA0V[`\x01`\x01`\xE0\x1B\x03\x91\x82\x16\x92\x91\x16\x90V[a!ZCa\x1E\xF4V[`\x01`\x01`\xE0\x1B\x03\x83\x11a\x19\xCCWa!@\x92`\x01`\x01`\xE0\x1B\x03\x16\x91a\"\xA0V[Q_\x91\x90\x82\x80[\x82Q\x82\x10\x15a!\xD1Wa!\x9Ba\t\xA6a\x12\xB3\x84\x86a\x10\xF0V[\x90`\x01`\x01`\xA0\x1B\x03\x80\x83\x16\x91\x16\x10\x15a\x1FqWa!\xC8`\x01\x91\x95a\x13\x94a\x13\x82` a\x13s\x87\x89a\x10\xF0V[\x91\x01\x90\x93a!\x82V[PPP\x90a'\x10\x14\x15_\x14a!\xE4W_\x90V[`\x01\x90V[\x80T\x90`\x01`@\x1B\x82\x10\x15a\x01\xD7W`\x01\x82\x01\x80\x82U\x82\x10\x15a\x11\x04W_\x90\x81R` \x90 \x01\x81Q` \x92\x83\x01Q\x90\x92\x1Bc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[\x90`@Qa\"?\x81a\x01\xF7V[` \x81\x93Tc\xFF\xFF\xFF\xFF\x81\x16\x83R\x81\x1C\x91\x01RV[\x15a\"[WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7FCheckpoint: decreasing keys\0\0\0\0\0`D\x82\x01R`d\x90\xFD[\x90\x92\x91\x92\x83\x82T\x80\x15\x15_\x14a#\x8CW\x92` \x92\x91\x84a\"\xD8a\"\xD3a\"\xC8a#R\x98a\x14\xB3V[\x85_R` _ \x01\x90V[a\"2V[\x93c\xFF\xFF\xFF\xFFa\"\xFDa\"\xEF\x87Qc\xFF\xFF\xFF\xFF\x16\x90V[\x82\x84\x16\x92\x83\x91\x16\x11\x15a\"TV[a#\x17a#\x0E\x87Qc\xFF\xFF\xFF\xFF\x16\x90V[c\xFF\xFF\xFF\xFF\x16\x90V[\x03a#VWPa#D\x92a\x16\xF7a#-\x92a\x14\xB3V[\x90c\xFF\xFF\xFF\xFF\x82T\x91\x81\x19\x90` \x1B\x16\x91\x16\x17\x90UV[\x01Q`\x01`\x01`\xE0\x1B\x03\x16\x90V[\x91\x90V[\x91PPa#\x87\x91a#ta#ha\x023V[c\xFF\xFF\xFF\xFF\x90\x93\x16\x83RV[`\x01`\x01`\xE0\x1B\x03\x88\x16\x82\x86\x01Ra!\xE9V[a#DV[PPa#\xB1\x91a#\x9Da#ha\x023V[`\x01`\x01`\xE0\x1B\x03\x85\x16` \x83\x01Ra!\xE9V[_\x91\x90V[\x90`A\x81Q\x14_\x14a#\xE2Wa#\xDE\x91` \x82\x01Q\x90```@\x84\x01Q\x93\x01Q_\x1A\x90a#\xFAV[\x90\x91V[PP_\x90`\x02\x90V[\x90\x81` \x91\x03\x12a\x02lWQ\x90V[\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x84\x11a$jW` \x93_\x93`\xFF`\x80\x94`@Q\x94\x85R\x16\x86\x84\x01R`@\x83\x01R``\x82\x01R\x82\x80R`\x01Z\xFA\x15a\n2W_Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a$bW\x90_\x90V[P_\x90`\x01\x90V[PPPP_\x90`\x03\x90V\xFEFP\x18y\xB8\xCA\x85%\xE8\xC2\xFDQ\x9E/\xBF\xCF\xA2\xEB\xEA&P\x12\x94\xAA\x02\xCB\xFC\xFB\x12\xE9CT\xA2dipfsX\"\x12 \x0B\xD94\xCA\x0C3\x15\x8AcN\x91\xF8\xEB\xA93`\ri\x93\xAA1\xA2f\x93R\xD7\x96\x9CT\xEC\xBB\x8EdsolcC\0\x08\x1B\x003",
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<OperatorAlreadyRegistered> for UnderlyingRustTuple<'_> {
            fn from(value: OperatorAlreadyRegistered) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OperatorAlreadyRegistered {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OperatorAlreadyRegistered {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
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
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "Initialized(uint8)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    127u8, 38u8, 184u8, 63u8, 249u8, 110u8, 31u8, 43u8, 106u8, 104u8, 47u8, 19u8,
                    56u8, 82u8, 246u8, 121u8, 138u8, 9u8, 196u8, 101u8, 218u8, 149u8, 146u8, 20u8,
                    96u8, 206u8, 251u8, 56u8, 71u8, 64u8, 36u8, 152u8,
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
                    return Err(alloy_sol_types::Error::invalid_event_signature_hash(
                        Self::SIGNATURE,
                        topics.0,
                        Self::SIGNATURE_HASH,
                    ));
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::tokenize(
                        &self.version,
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
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
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "MinimumWeightUpdated(uint256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    113u8, 60u8, 165u8, 59u8, 136u8, 214u8, 235u8, 99u8, 245u8, 177u8, 133u8, 76u8,
                    184u8, 203u8, 221u8, 115u8, 110u8, 197u8, 30u8, 218u8, 34u8, 94u8, 70u8, 121u8,
                    26u8, 169u8, 41u8, 139u8, 1u8, 96u8, 100u8, 143u8,
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
                    return Err(alloy_sol_types::Error::invalid_event_signature_hash(
                        Self::SIGNATURE,
                        topics.0,
                        Self::SIGNATURE_HASH,
                    ));
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.previous,
                    ),
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
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
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "OperatorDeregistered(address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    49u8, 224u8, 173u8, 254u8, 199u8, 27u8, 204u8, 238u8, 55u8, 182u8, 232u8, 58u8,
                    144u8, 194u8, 254u8, 219u8, 23u8, 216u8, 241u8, 105u8, 63u8, 238u8, 134u8,
                    60u8, 71u8, 113u8, 231u8, 191u8, 226u8, 174u8, 213u8, 128u8,
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
                    return Err(alloy_sol_types::Error::invalid_event_signature_hash(
                        Self::SIGNATURE,
                        topics.0,
                        Self::SIGNATURE_HASH,
                    ));
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
                    self.operator.clone(),
                    self.avs.clone(),
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
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
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "OperatorRegistered(address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    164u8, 83u8, 219u8, 97u8, 42u8, 245u8, 158u8, 85u8, 33u8, 214u8, 171u8, 146u8,
                    132u8, 220u8, 62u8, 45u8, 6u8, 175u8, 40u8, 110u8, 177u8, 177u8, 183u8, 183u8,
                    113u8, 252u8, 228u8, 113u8, 108u8, 25u8, 242u8, 193u8,
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
                    return Err(alloy_sol_types::Error::invalid_event_signature_hash(
                        Self::SIGNATURE,
                        topics.0,
                        Self::SIGNATURE_HASH,
                    ));
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
                    self.operator.clone(),
                    self.avs.clone(),
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
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
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "OperatorWeightUpdated(address,uint256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    136u8, 119u8, 13u8, 200u8, 98u8, 228u8, 122u8, 126u8, 213u8, 134u8, 144u8,
                    120u8, 87u8, 235u8, 27u8, 117u8, 228u8, 197u8, 255u8, 200u8, 183u8, 7u8, 199u8,
                    238u8, 16u8, 235u8, 116u8, 214u8, 136u8, 95u8, 229u8, 148u8,
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
                    return Err(alloy_sol_types::Error::invalid_event_signature_hash(
                        Self::SIGNATURE,
                        topics.0,
                        Self::SIGNATURE_HASH,
                    ));
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.oldWeight,
                    ),
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.newWeight,
                    ),
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
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
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "OwnershipTransferred(address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    139u8, 224u8, 7u8, 156u8, 83u8, 22u8, 89u8, 20u8, 19u8, 68u8, 205u8, 31u8,
                    208u8, 164u8, 242u8, 132u8, 25u8, 73u8, 127u8, 151u8, 34u8, 163u8, 218u8,
                    175u8, 227u8, 180u8, 24u8, 111u8, 107u8, 100u8, 87u8, 224u8,
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
                    return Err(alloy_sol_types::Error::invalid_event_signature_hash(
                        Self::SIGNATURE,
                        topics.0,
                        Self::SIGNATURE_HASH,
                    ));
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
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
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str =
                "QuorumUpdated(((address,uint96)[]),((address,uint96)[]))";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    35u8, 170u8, 212u8, 230u8, 23u8, 68u8, 236u8, 225u8, 100u8, 19u8, 10u8, 164u8,
                    21u8, 193u8, 97u8, 110u8, 128u8, 19u8, 107u8, 15u8, 7u8, 112u8, 229u8, 101u8,
                    137u8, 67u8, 139u8, 144u8, 178u8, 105u8, 38u8, 94u8,
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
                    return Err(alloy_sol_types::Error::invalid_event_signature_hash(
                        Self::SIGNATURE,
                        topics.0,
                        Self::SIGNATURE_HASH,
                    ));
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
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
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "SigningKeyUpdate(address,uint256,address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    208u8, 97u8, 22u8, 130u8, 82u8, 244u8, 65u8, 115u8, 54u8, 88u8, 240u8, 158u8,
                    77u8, 143u8, 91u8, 45u8, 153u8, 142u8, 212u8, 239u8, 36u8, 162u8, 187u8, 253u8,
                    108u8, 236u8, 165u8, 46u8, 161u8, 49u8, 80u8, 2u8,
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
                    return Err(alloy_sol_types::Error::invalid_event_signature_hash(
                        Self::SIGNATURE,
                        topics.0,
                        Self::SIGNATURE_HASH,
                    ));
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
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
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "ThresholdWeightUpdated(uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    147u8, 36u8, 247u8, 229u8, 167u8, 192u8, 40u8, 136u8, 8u8, 166u8, 52u8, 204u8,
                    222u8, 68u8, 184u8, 233u8, 121u8, 103u8, 100u8, 116u8, 178u8, 46u8, 41u8,
                    238u8, 157u8, 213u8, 105u8, 181u8, 94u8, 121u8, 26u8, 75u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    thresholdWeight: data.0,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(alloy_sol_types::Error::invalid_event_signature_hash(
                        Self::SIGNATURE,
                        topics.0,
                        Self::SIGNATURE_HASH,
                    ));
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.thresholdWeight,
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
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
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "TotalWeightUpdated(uint256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    134u8, 220u8, 248u8, 107u8, 18u8, 223u8, 238u8, 222u8, 167u8, 74u8, 233u8,
                    48u8, 13u8, 189u8, 170u8, 25u8, 59u8, 204u8, 229u8, 128u8, 147u8, 105u8, 200u8,
                    23u8, 126u8, 162u8, 244u8, 234u8, 170u8, 101u8, 114u8, 155u8,
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
                    return Err(alloy_sol_types::Error::invalid_event_signature_hash(
                        Self::SIGNATURE,
                        topics.0,
                        Self::SIGNATURE_HASH,
                    ));
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.oldTotalWeight,
                    ),
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.newTotalWeight,
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
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
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "UpdateMinimumWeight(uint256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    30u8, 164u8, 33u8, 134u8, 179u8, 5u8, 250u8, 55u8, 49u8, 4u8, 80u8, 217u8,
                    251u8, 135u8, 234u8, 30u8, 143u8, 12u8, 127u8, 68u8, 126u8, 119u8, 20u8, 121u8,
                    227u8, 178u8, 118u8, 52u8, 191u8, 232u8, 77u8, 193u8,
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
                    return Err(alloy_sol_types::Error::invalid_event_signature_hash(
                        Self::SIGNATURE,
                        topics.0,
                        Self::SIGNATURE_HASH,
                    ));
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.oldMinimumWeight,
                    ),
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.newMinimumWeight,
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<deregisterOperatorCall> for UnderlyingRustTuple<'_> {
                fn from(value: deregisterOperatorCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for deregisterOperatorCall {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<deregisterOperatorReturn> for UnderlyingRustTuple<'_> {
                fn from(value: deregisterOperatorReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for deregisterOperatorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for deregisterOperatorCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = deregisterOperatorReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getLastCheckpointOperatorWeightCall> for UnderlyingRustTuple<'_> {
                fn from(value: getLastCheckpointOperatorWeightCall) -> Self {
                    (value.operator,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getLastCheckpointOperatorWeightCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { operator: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U256,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getLastCheckpointOperatorWeightReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getLastCheckpointOperatorWeightReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getLastCheckpointOperatorWeightReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getLastCheckpointOperatorWeightCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getLastCheckpointOperatorWeightReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getLastCheckpointThresholdWeightCall> for UnderlyingRustTuple<'_> {
                fn from(value: getLastCheckpointThresholdWeightCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getLastCheckpointThresholdWeightCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U256,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getLastCheckpointThresholdWeightReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getLastCheckpointThresholdWeightReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getLastCheckpointThresholdWeightReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getLastCheckpointThresholdWeightCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getLastCheckpointThresholdWeightReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getLastCheckpointThresholdWeightAtBlockCall>
                for UnderlyingRustTuple<'_>
            {
                fn from(value: getLastCheckpointThresholdWeightAtBlockCall) -> Self {
                    (value.blockNumber,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
                for getLastCheckpointThresholdWeightAtBlockCall
            {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        blockNumber: tuple.0,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U256,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getLastCheckpointThresholdWeightAtBlockReturn>
                for UnderlyingRustTuple<'_>
            {
                fn from(value: getLastCheckpointThresholdWeightAtBlockReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
                for getLastCheckpointThresholdWeightAtBlockReturn
            {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getLastCheckpointThresholdWeightAtBlockCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getLastCheckpointThresholdWeightAtBlockReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                    <alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::SolType>::tokenize(
                        &self.blockNumber,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getLastCheckpointTotalWeightCall> for UnderlyingRustTuple<'_> {
                fn from(value: getLastCheckpointTotalWeightCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getLastCheckpointTotalWeightCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U256,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getLastCheckpointTotalWeightReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getLastCheckpointTotalWeightReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getLastCheckpointTotalWeightReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getLastCheckpointTotalWeightCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getLastCheckpointTotalWeightReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getLastCheckpointTotalWeightAtBlockCall> for UnderlyingRustTuple<'_> {
                fn from(value: getLastCheckpointTotalWeightAtBlockCall) -> Self {
                    (value.blockNumber,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getLastCheckpointTotalWeightAtBlockCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        blockNumber: tuple.0,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U256,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getLastCheckpointTotalWeightAtBlockReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getLastCheckpointTotalWeightAtBlockReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getLastCheckpointTotalWeightAtBlockReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getLastCheckpointTotalWeightAtBlockCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getLastCheckpointTotalWeightAtBlockReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                    <alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::SolType>::tokenize(
                        &self.blockNumber,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getLatestOperatorSigningKeyCall> for UnderlyingRustTuple<'_> {
                fn from(value: getLatestOperatorSigningKeyCall) -> Self {
                    (value.operator,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getLatestOperatorSigningKeyCall {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getLatestOperatorSigningKeyReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getLatestOperatorSigningKeyReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getLatestOperatorSigningKeyReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getLatestOperatorSigningKeyCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getLatestOperatorSigningKeyReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getOperatorSigningKeyAtBlockCall> for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorSigningKeyAtBlockCall) -> Self {
                    (value.operator, value.blockNumber)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getOperatorSigningKeyAtBlockCall {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getOperatorSigningKeyAtBlockReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorSigningKeyAtBlockReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getOperatorSigningKeyAtBlockReturn {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getOperatorSigningKeyAtBlockReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.blockNumber,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getOperatorWeightCall> for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorWeightCall) -> Self {
                    (value.operator,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getOperatorWeightCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { operator: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U256,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getOperatorWeightReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorWeightReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getOperatorWeightReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getOperatorWeightCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getOperatorWeightReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getOperatorWeightAtBlockCall> for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorWeightAtBlockCall) -> Self {
                    (value.operator, value.blockNumber)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getOperatorWeightAtBlockCall {
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
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U256,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getOperatorWeightAtBlockReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorWeightAtBlockReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getOperatorWeightAtBlockReturn {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getOperatorWeightAtBlockReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                    <alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::SolType>::tokenize(
                        &self.blockNumber,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = initializeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.thresholdWeight,
                    ),
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<isValidSignatureCall> for UnderlyingRustTuple<'_> {
                fn from(value: isValidSignatureCall) -> Self {
                    (value.digest, value._signatureData)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isValidSignatureCall {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<isValidSignatureReturn> for UnderlyingRustTuple<'_> {
                fn from(value: isValidSignatureReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isValidSignatureReturn {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = isValidSignatureReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<4>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U256,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = minimumWeightReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<operatorRegisteredCall> for UnderlyingRustTuple<'_> {
                fn from(value: operatorRegisteredCall) -> Self {
                    (value.operator,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for operatorRegisteredCall {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<operatorRegisteredReturn> for UnderlyingRustTuple<'_> {
                fn from(value: operatorRegisteredReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for operatorRegisteredReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for operatorRegisteredCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = operatorRegisteredReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = ownerReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type UnderlyingRustTuple<'a> =
                (<IECDSAStakeRegistryTypes::Quorum as alloy::sol_types::SolType>::RustType,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = quorumReturn;
            type ReturnTuple<'a> = (IECDSAStakeRegistryTypes::Quorum,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `registerOperatorWithSignature((bytes,bytes32,uint256),address)` and selector `0x3d5611f6`.
    ```solidity
    function registerOperatorWithSignature(ISignatureUtilsMixinTypes.SignatureWithSaltAndExpiry memory operatorSignature, address signingKey) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct registerOperatorWithSignatureCall {
        #[allow(missing_docs)]
        pub operatorSignature: <ISignatureUtilsMixinTypes::SignatureWithSaltAndExpiry as alloy::sol_types::SolType>::RustType,
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
                ISignatureUtilsMixinTypes::SignatureWithSaltAndExpiry,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <ISignatureUtilsMixinTypes::SignatureWithSaltAndExpiry as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::Address,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<registerOperatorWithSignatureCall> for UnderlyingRustTuple<'_> {
                fn from(value: registerOperatorWithSignatureCall) -> Self {
                    (value.operatorSignature, value.signingKey)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for registerOperatorWithSignatureCall {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<registerOperatorWithSignatureReturn> for UnderlyingRustTuple<'_> {
                fn from(value: registerOperatorWithSignatureReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for registerOperatorWithSignatureReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for registerOperatorWithSignatureCall {
            type Parameters<'a> = (
                ISignatureUtilsMixinTypes::SignatureWithSaltAndExpiry,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = registerOperatorWithSignatureReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str =
                "registerOperatorWithSignature((bytes,bytes32,uint256),address)";
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
                    <ISignatureUtilsMixinTypes::SignatureWithSaltAndExpiry as alloy_sol_types::SolType>::tokenize(
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<renounceOwnershipCall> for UnderlyingRustTuple<'_> {
                fn from(value: renounceOwnershipCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for renounceOwnershipCall {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<renounceOwnershipReturn> for UnderlyingRustTuple<'_> {
                fn from(value: renounceOwnershipReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for renounceOwnershipReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for renounceOwnershipCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = renounceOwnershipReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<transferOwnershipCall> for UnderlyingRustTuple<'_> {
                fn from(value: transferOwnershipCall) -> Self {
                    (value.newOwner,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for transferOwnershipCall {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<transferOwnershipReturn> for UnderlyingRustTuple<'_> {
                fn from(value: transferOwnershipReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for transferOwnershipReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for transferOwnershipCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = transferOwnershipReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
        pub operators: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<updateMinimumWeightCall> for UnderlyingRustTuple<'_> {
                fn from(value: updateMinimumWeightCall) -> Self {
                    (value.newMinimumWeight, value.operators)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for updateMinimumWeightCall {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<updateMinimumWeightReturn> for UnderlyingRustTuple<'_> {
                fn from(value: updateMinimumWeightReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for updateMinimumWeightReturn {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = updateMinimumWeightReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<updateOperatorSigningKeyCall> for UnderlyingRustTuple<'_> {
                fn from(value: updateOperatorSigningKeyCall) -> Self {
                    (value.newSigningKey,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for updateOperatorSigningKeyCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        newSigningKey: tuple.0,
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<updateOperatorSigningKeyReturn> for UnderlyingRustTuple<'_> {
                fn from(value: updateOperatorSigningKeyReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for updateOperatorSigningKeyReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for updateOperatorSigningKeyCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = updateOperatorSigningKeyReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
        pub operators: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
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
            type UnderlyingSolTuple<'a> =
                (alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> =
                (alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<updateOperatorsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: updateOperatorsReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for updateOperatorsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for updateOperatorsCall {
            type Parameters<'a> =
                (alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = updateOperatorsReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                (<alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Address,
                > as alloy_sol_types::SolType>::tokenize(
                    &self.operators
                ),)
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
                    alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<updateOperatorsForQuorumCall> for UnderlyingRustTuple<'_> {
                fn from(value: updateOperatorsForQuorumCall) -> Self {
                    (value.operatorsPerQuorum, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for updateOperatorsForQuorumCall {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<updateOperatorsForQuorumReturn> for UnderlyingRustTuple<'_> {
                fn from(value: updateOperatorsForQuorumReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for updateOperatorsForQuorumReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for updateOperatorsForQuorumCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                >,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = updateOperatorsForQuorumReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                        alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.operatorsPerQuorum
                    ),
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
        pub operators: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<updateQuorumConfigCall> for UnderlyingRustTuple<'_> {
                fn from(value: updateQuorumConfigCall) -> Self {
                    (value.quorum, value.operators)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for updateQuorumConfigCall {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<updateQuorumConfigReturn> for UnderlyingRustTuple<'_> {
                fn from(value: updateQuorumConfigReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for updateQuorumConfigReturn {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = updateQuorumConfigReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U256,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<updateStakeThresholdCall> for UnderlyingRustTuple<'_> {
                fn from(value: updateStakeThresholdCall) -> Self {
                    (value.thresholdWeight,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for updateStakeThresholdCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        thresholdWeight: tuple.0,
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<updateStakeThresholdReturn> for UnderlyingRustTuple<'_> {
                fn from(value: updateStakeThresholdReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for updateStakeThresholdReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for updateStakeThresholdCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = updateStakeThresholdReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.thresholdWeight,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
        getLastCheckpointThresholdWeightAtBlock(getLastCheckpointThresholdWeightAtBlockCall),
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
            )
                -> alloy_sol_types::Result<ECDSAStakeRegistryCalls>] = &[
                {
                    fn updateOperators(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryCalls> {
                        <updateOperatorsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
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
                            data, validate,
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
                        <quorumCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
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
                            data, validate,
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
                            data, validate,
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
                            data, validate,
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
                            data, validate,
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
                            data, validate,
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
                            data, validate,
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
                            data, validate,
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
                        <ownerCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
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
                            data, validate,
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
                            data, validate,
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
                        <initializeCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
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
                            data, validate,
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
                            data, validate,
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
                            data, validate,
                        )
                        .map(ECDSAStakeRegistryCalls::transferOwnership)
                    }
                    transferOwnership
                },
            ];
            let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                return Err(alloy_sol_types::Error::unknown_selector(
                    <Self as alloy_sol_types::SolInterface>::NAME,
                    selector,
                ));
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
                Self::InvalidLength(_) => <InvalidLength as alloy_sol_types::SolError>::SELECTOR,
                Self::InvalidQuorum(_) => <InvalidQuorum as alloy_sol_types::SolError>::SELECTOR,
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
                Self::LengthMismatch(_) => <LengthMismatch as alloy_sol_types::SolError>::SELECTOR,
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
            )
                -> alloy_sol_types::Result<ECDSAStakeRegistryErrors>] = &[
                {
                    fn OperatorNotRegistered(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryErrors> {
                        <OperatorNotRegistered as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
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
                            data, validate,
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
                            data, validate,
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
                            data, validate,
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
                        <InvalidLength as alloy_sol_types::SolError>::abi_decode_raw(data, validate)
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
                            data, validate,
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
                            data, validate,
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
                            data, validate,
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
                        <NotSorted as alloy_sol_types::SolError>::abi_decode_raw(data, validate)
                            .map(ECDSAStakeRegistryErrors::NotSorted)
                    }
                    NotSorted
                },
                {
                    fn InvalidQuorum(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryErrors> {
                        <InvalidQuorum as alloy_sol_types::SolError>::abi_decode_raw(data, validate)
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
                            data, validate,
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
                            data, validate,
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
                            data, validate,
                        )
                        .map(ECDSAStakeRegistryErrors::LengthMismatch)
                    }
                    LengthMismatch
                },
            ];
            let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                return Err(alloy_sol_types::Error::unknown_selector(
                    <Self as alloy_sol_types::SolInterface>::NAME,
                    selector,
                ));
            };
            DECODE_SHIMS[idx](data, validate)
        }
        #[inline]
        fn abi_encoded_size(&self) -> usize {
            match self {
                Self::InsufficientSignedStake(inner) => {
                    <InsufficientSignedStake as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InsufficientWeight(inner) => {
                    <InsufficientWeight as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InvalidLength(inner) => {
                    <InvalidLength as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InvalidQuorum(inner) => {
                    <InvalidQuorum as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InvalidReferenceBlock(inner) => {
                    <InvalidReferenceBlock as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InvalidSignature(inner) => {
                    <InvalidSignature as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InvalidSignedWeight(inner) => {
                    <InvalidSignedWeight as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InvalidThreshold(inner) => {
                    <InvalidThreshold as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::LengthMismatch(inner) => {
                    <LengthMismatch as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::MustUpdateAllOperators(inner) => {
                    <MustUpdateAllOperators as alloy_sol_types::SolError>::abi_encoded_size(inner)
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
                    <OperatorNotRegistered as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::InsufficientSignedStake(inner) => {
                    <InsufficientSignedStake as alloy_sol_types::SolError>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::InsufficientWeight(inner) => {
                    <InsufficientWeight as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::InvalidLength(inner) => {
                    <InvalidLength as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::InvalidQuorum(inner) => {
                    <InvalidQuorum as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::InvalidReferenceBlock(inner) => {
                    <InvalidReferenceBlock as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::InvalidSignature(inner) => {
                    <InvalidSignature as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::InvalidSignedWeight(inner) => {
                    <InvalidSignedWeight as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::InvalidThreshold(inner) => {
                    <InvalidThreshold as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::LengthMismatch(inner) => {
                    <LengthMismatch as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::MustUpdateAllOperators(inner) => {
                    <MustUpdateAllOperators as alloy_sol_types::SolError>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::NotSorted(inner) => {
                    <NotSorted as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::OperatorAlreadyRegistered(inner) => {
                    <OperatorAlreadyRegistered as alloy_sol_types::SolError>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::OperatorNotRegistered(inner) => {
                    <OperatorNotRegistered as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
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
                30u8, 164u8, 33u8, 134u8, 179u8, 5u8, 250u8, 55u8, 49u8, 4u8, 80u8, 217u8, 251u8,
                135u8, 234u8, 30u8, 143u8, 12u8, 127u8, 68u8, 126u8, 119u8, 20u8, 121u8, 227u8,
                178u8, 118u8, 52u8, 191u8, 232u8, 77u8, 193u8,
            ],
            [
                35u8, 170u8, 212u8, 230u8, 23u8, 68u8, 236u8, 225u8, 100u8, 19u8, 10u8, 164u8,
                21u8, 193u8, 97u8, 110u8, 128u8, 19u8, 107u8, 15u8, 7u8, 112u8, 229u8, 101u8,
                137u8, 67u8, 139u8, 144u8, 178u8, 105u8, 38u8, 94u8,
            ],
            [
                49u8, 224u8, 173u8, 254u8, 199u8, 27u8, 204u8, 238u8, 55u8, 182u8, 232u8, 58u8,
                144u8, 194u8, 254u8, 219u8, 23u8, 216u8, 241u8, 105u8, 63u8, 238u8, 134u8, 60u8,
                71u8, 113u8, 231u8, 191u8, 226u8, 174u8, 213u8, 128u8,
            ],
            [
                113u8, 60u8, 165u8, 59u8, 136u8, 214u8, 235u8, 99u8, 245u8, 177u8, 133u8, 76u8,
                184u8, 203u8, 221u8, 115u8, 110u8, 197u8, 30u8, 218u8, 34u8, 94u8, 70u8, 121u8,
                26u8, 169u8, 41u8, 139u8, 1u8, 96u8, 100u8, 143u8,
            ],
            [
                127u8, 38u8, 184u8, 63u8, 249u8, 110u8, 31u8, 43u8, 106u8, 104u8, 47u8, 19u8, 56u8,
                82u8, 246u8, 121u8, 138u8, 9u8, 196u8, 101u8, 218u8, 149u8, 146u8, 20u8, 96u8,
                206u8, 251u8, 56u8, 71u8, 64u8, 36u8, 152u8,
            ],
            [
                134u8, 220u8, 248u8, 107u8, 18u8, 223u8, 238u8, 222u8, 167u8, 74u8, 233u8, 48u8,
                13u8, 189u8, 170u8, 25u8, 59u8, 204u8, 229u8, 128u8, 147u8, 105u8, 200u8, 23u8,
                126u8, 162u8, 244u8, 234u8, 170u8, 101u8, 114u8, 155u8,
            ],
            [
                136u8, 119u8, 13u8, 200u8, 98u8, 228u8, 122u8, 126u8, 213u8, 134u8, 144u8, 120u8,
                87u8, 235u8, 27u8, 117u8, 228u8, 197u8, 255u8, 200u8, 183u8, 7u8, 199u8, 238u8,
                16u8, 235u8, 116u8, 214u8, 136u8, 95u8, 229u8, 148u8,
            ],
            [
                139u8, 224u8, 7u8, 156u8, 83u8, 22u8, 89u8, 20u8, 19u8, 68u8, 205u8, 31u8, 208u8,
                164u8, 242u8, 132u8, 25u8, 73u8, 127u8, 151u8, 34u8, 163u8, 218u8, 175u8, 227u8,
                180u8, 24u8, 111u8, 107u8, 100u8, 87u8, 224u8,
            ],
            [
                147u8, 36u8, 247u8, 229u8, 167u8, 192u8, 40u8, 136u8, 8u8, 166u8, 52u8, 204u8,
                222u8, 68u8, 184u8, 233u8, 121u8, 103u8, 100u8, 116u8, 178u8, 46u8, 41u8, 238u8,
                157u8, 213u8, 105u8, 181u8, 94u8, 121u8, 26u8, 75u8,
            ],
            [
                164u8, 83u8, 219u8, 97u8, 42u8, 245u8, 158u8, 85u8, 33u8, 214u8, 171u8, 146u8,
                132u8, 220u8, 62u8, 45u8, 6u8, 175u8, 40u8, 110u8, 177u8, 177u8, 183u8, 183u8,
                113u8, 252u8, 228u8, 113u8, 108u8, 25u8, 242u8, 193u8,
            ],
            [
                208u8, 97u8, 22u8, 130u8, 82u8, 244u8, 65u8, 115u8, 54u8, 88u8, 240u8, 158u8, 77u8,
                143u8, 91u8, 45u8, 153u8, 142u8, 212u8, 239u8, 36u8, 162u8, 187u8, 253u8, 108u8,
                236u8, 165u8, 46u8, 161u8, 49u8, 80u8, 2u8,
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
                        topics, data, validate,
                    )
                    .map(Self::Initialized)
                }
                Some(<MinimumWeightUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <MinimumWeightUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::MinimumWeightUpdated)
                }
                Some(<OperatorDeregistered as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <OperatorDeregistered as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::OperatorDeregistered)
                }
                Some(<OperatorRegistered as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <OperatorRegistered as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::OperatorRegistered)
                }
                Some(<OperatorWeightUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <OperatorWeightUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::OperatorWeightUpdated)
                }
                Some(<OwnershipTransferred as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <OwnershipTransferred as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::OwnershipTransferred)
                }
                Some(<QuorumUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <QuorumUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::QuorumUpdated)
                }
                Some(<SigningKeyUpdate as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <SigningKeyUpdate as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::SigningKeyUpdate)
                }
                Some(<ThresholdWeightUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <ThresholdWeightUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::ThresholdWeightUpdated)
                }
                Some(<TotalWeightUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <TotalWeightUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::TotalWeightUpdated)
                }
                Some(<UpdateMinimumWeight as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <UpdateMinimumWeight as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::UpdateMinimumWeight)
                }
                _ => alloy_sol_types::private::Err(alloy_sol_types::Error::InvalidLog {
                    name: <Self as alloy_sol_types::SolEventInterface>::NAME,
                    log: alloy_sol_types::private::Box::new(
                        alloy_sol_types::private::LogData::new_unchecked(
                            topics.to_vec(),
                            data.to_vec().into(),
                        ),
                    ),
                }),
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
    ) -> impl ::core::future::Future<Output = alloy_contract::Result<ECDSAStakeRegistryInstance<T, P, N>>>
    {
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
        ECDSAStakeRegistryInstance::<T, P, N>::deploy_builder(provider, _delegationManager)
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
            f.debug_tuple("ECDSAStakeRegistryInstance")
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
        > ECDSAStakeRegistryInstance<T, P, N>
    {
        /**Creates a new wrapper around an on-chain [`ECDSAStakeRegistry`](self) contract instance.

        See the [wrapper's documentation](`ECDSAStakeRegistryInstance`) for more details.*/
        #[inline]
        pub const fn new(address: alloy_sol_types::private::Address, provider: P) -> Self {
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
                    &alloy_sol_types::SolConstructor::abi_encode(&constructorCall {
                        _delegationManager,
                    })[..],
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
        > ECDSAStakeRegistryInstance<T, P, N>
    {
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
        ) -> alloy_contract::SolCallBuilder<T, &P, getLastCheckpointOperatorWeightCall, N> {
            self.call_builder(&getLastCheckpointOperatorWeightCall { operator })
        }
        ///Creates a new call builder for the [`getLastCheckpointThresholdWeight`] function.
        pub fn getLastCheckpointThresholdWeight(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getLastCheckpointThresholdWeightCall, N>
        {
            self.call_builder(&getLastCheckpointThresholdWeightCall {})
        }
        ///Creates a new call builder for the [`getLastCheckpointThresholdWeightAtBlock`] function.
        pub fn getLastCheckpointThresholdWeightAtBlock(
            &self,
            blockNumber: u32,
        ) -> alloy_contract::SolCallBuilder<T, &P, getLastCheckpointThresholdWeightAtBlockCall, N>
        {
            self.call_builder(&getLastCheckpointThresholdWeightAtBlockCall { blockNumber })
        }
        ///Creates a new call builder for the [`getLastCheckpointTotalWeight`] function.
        pub fn getLastCheckpointTotalWeight(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getLastCheckpointTotalWeightCall, N> {
            self.call_builder(&getLastCheckpointTotalWeightCall {})
        }
        ///Creates a new call builder for the [`getLastCheckpointTotalWeightAtBlock`] function.
        pub fn getLastCheckpointTotalWeightAtBlock(
            &self,
            blockNumber: u32,
        ) -> alloy_contract::SolCallBuilder<T, &P, getLastCheckpointTotalWeightAtBlockCall, N>
        {
            self.call_builder(&getLastCheckpointTotalWeightAtBlockCall { blockNumber })
        }
        ///Creates a new call builder for the [`getLatestOperatorSigningKey`] function.
        pub fn getLatestOperatorSigningKey(
            &self,
            operator: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, getLatestOperatorSigningKeyCall, N> {
            self.call_builder(&getLatestOperatorSigningKeyCall { operator })
        }
        ///Creates a new call builder for the [`getOperatorSigningKeyAtBlock`] function.
        pub fn getOperatorSigningKeyAtBlock(
            &self,
            operator: alloy::sol_types::private::Address,
            blockNumber: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, getOperatorSigningKeyAtBlockCall, N> {
            self.call_builder(&getOperatorSigningKeyAtBlockCall {
                operator,
                blockNumber,
            })
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
            self.call_builder(&getOperatorWeightAtBlockCall {
                operator,
                blockNumber,
            })
        }
        ///Creates a new call builder for the [`initialize`] function.
        pub fn initialize(
            &self,
            _serviceManager: alloy::sol_types::private::Address,
            thresholdWeight: alloy::sol_types::private::primitives::aliases::U256,
            quorum: <IECDSAStakeRegistryTypes::Quorum as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, initializeCall, N> {
            self.call_builder(&initializeCall {
                _serviceManager,
                thresholdWeight,
                quorum,
            })
        }
        ///Creates a new call builder for the [`isValidSignature`] function.
        pub fn isValidSignature(
            &self,
            digest: alloy::sol_types::private::FixedBytes<32>,
            _signatureData: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, isValidSignatureCall, N> {
            self.call_builder(&isValidSignatureCall {
                digest,
                _signatureData,
            })
        }
        ///Creates a new call builder for the [`minimumWeight`] function.
        pub fn minimumWeight(&self) -> alloy_contract::SolCallBuilder<T, &P, minimumWeightCall, N> {
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
            operatorSignature: <ISignatureUtilsMixinTypes::SignatureWithSaltAndExpiry as alloy::sol_types::SolType>::RustType,
            signingKey: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, registerOperatorWithSignatureCall, N> {
            self.call_builder(&registerOperatorWithSignatureCall {
                operatorSignature,
                signingKey,
            })
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
            self.call_builder(&updateMinimumWeightCall {
                newMinimumWeight,
                operators,
            })
        }
        ///Creates a new call builder for the [`updateOperatorSigningKey`] function.
        pub fn updateOperatorSigningKey(
            &self,
            newSigningKey: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, updateOperatorSigningKeyCall, N> {
            self.call_builder(&updateOperatorSigningKeyCall { newSigningKey })
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
            self.call_builder(&updateOperatorsForQuorumCall {
                operatorsPerQuorum,
                _1,
            })
        }
        ///Creates a new call builder for the [`updateQuorumConfig`] function.
        pub fn updateQuorumConfig(
            &self,
            quorum: <IECDSAStakeRegistryTypes::Quorum as alloy::sol_types::SolType>::RustType,
            operators: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
        ) -> alloy_contract::SolCallBuilder<T, &P, updateQuorumConfigCall, N> {
            self.call_builder(&updateQuorumConfigCall { quorum, operators })
        }
        ///Creates a new call builder for the [`updateStakeThreshold`] function.
        pub fn updateStakeThreshold(
            &self,
            thresholdWeight: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, updateStakeThresholdCall, N> {
            self.call_builder(&updateStakeThresholdCall { thresholdWeight })
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
            T: alloy_contract::private::Transport + ::core::clone::Clone,
            P: alloy_contract::private::Provider<T, N>,
            N: alloy_contract::private::Network,
        > ECDSAStakeRegistryInstance<T, P, N>
    {
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
        pub fn Initialized_filter(&self) -> alloy_contract::Event<T, &P, Initialized, N> {
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
        pub fn QuorumUpdated_filter(&self) -> alloy_contract::Event<T, &P, QuorumUpdated, N> {
            self.event_filter::<QuorumUpdated>()
        }
        ///Creates a new event filter for the [`SigningKeyUpdate`] event.
        pub fn SigningKeyUpdate_filter(&self) -> alloy_contract::Event<T, &P, SigningKeyUpdate, N> {
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
