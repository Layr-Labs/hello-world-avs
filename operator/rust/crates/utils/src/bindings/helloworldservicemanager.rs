///Module containing a contract's types and functions.
/**

```solidity
library IHelloWorldServiceManager {
    struct Task { string name; uint32 taskCreatedBlock; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod IHelloWorldServiceManager {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /**```solidity
    struct Task { string name; uint32 taskCreatedBlock; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone, Debug)]
    pub struct Task {
        #[allow(missing_docs)]
        pub name: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub taskCreatedBlock: u32,
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
            alloy::sol_types::sol_data::String,
            alloy::sol_types::sol_data::Uint<32>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (alloy::sol_types::private::String, u32);
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
        impl ::core::convert::From<Task> for UnderlyingRustTuple<'_> {
            fn from(value: Task) -> Self {
                (value.name, value.taskCreatedBlock)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for Task {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    name: tuple.0,
                    taskCreatedBlock: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for Task {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for Task {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.name,
                    ),
                    <alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::SolType>::tokenize(
                        &self.taskCreatedBlock,
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
        impl alloy_sol_types::SolType for Task {
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
        impl alloy_sol_types::SolStruct for Task {
            const NAME: &'static str = "Task";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed("Task(string name,uint32 taskCreatedBlock)")
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
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::eip712_data_word(
                            &self.name,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.taskCreatedBlock,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for Task {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::String as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.name,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.taskCreatedBlock,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
                <alloy::sol_types::sol_data::String as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.name,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.taskCreatedBlock,
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
    /**Creates a new wrapper around an on-chain [`IHelloWorldServiceManager`](self) contract instance.

    See the [wrapper's documentation](`IHelloWorldServiceManagerInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> IHelloWorldServiceManagerInstance<T, P, N> {
        IHelloWorldServiceManagerInstance::<T, P, N>::new(address, provider)
    }
    /**A [`IHelloWorldServiceManager`](self) instance.

    Contains type-safe methods for interacting with an on-chain instance of the
    [`IHelloWorldServiceManager`](self) contract located at a given `address`, using a given
    provider `P`.

    If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
    documentation on how to provide it), the `deploy` and `deploy_builder` methods can
    be used to deploy a new instance of the contract.

    See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct IHelloWorldServiceManagerInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for IHelloWorldServiceManagerInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("IHelloWorldServiceManagerInstance")
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
        > IHelloWorldServiceManagerInstance<T, P, N>
    {
        /**Creates a new wrapper around an on-chain [`IHelloWorldServiceManager`](self) contract instance.

        See the [wrapper's documentation](`IHelloWorldServiceManagerInstance`) for more details.*/
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
    impl<T, P: ::core::clone::Clone, N> IHelloWorldServiceManagerInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> IHelloWorldServiceManagerInstance<T, P, N> {
            IHelloWorldServiceManagerInstance {
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
        > IHelloWorldServiceManagerInstance<T, P, N>
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
        > IHelloWorldServiceManagerInstance<T, P, N>
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
library IRewardsCoordinatorTypes {
    struct OperatorDirectedRewardsSubmission { StrategyAndMultiplier[] strategiesAndMultipliers; address token; OperatorReward[] operatorRewards; uint32 startTimestamp; uint32 duration; string description; }
    struct OperatorReward { address operator; uint256 amount; }
    struct RewardsSubmission { StrategyAndMultiplier[] strategiesAndMultipliers; address token; uint256 amount; uint32 startTimestamp; uint32 duration; }
    struct StrategyAndMultiplier { address strategy; uint96 multiplier; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod IRewardsCoordinatorTypes {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /**```solidity
    struct OperatorDirectedRewardsSubmission { StrategyAndMultiplier[] strategiesAndMultipliers; address token; OperatorReward[] operatorRewards; uint32 startTimestamp; uint32 duration; string description; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OperatorDirectedRewardsSubmission {
        #[allow(missing_docs)]
        pub strategiesAndMultipliers: alloy::sol_types::private::Vec<
            <StrategyAndMultiplier as alloy::sol_types::SolType>::RustType,
        >,
        #[allow(missing_docs)]
        pub token: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub operatorRewards:
            alloy::sol_types::private::Vec<<OperatorReward as alloy::sol_types::SolType>::RustType>,
        #[allow(missing_docs)]
        pub startTimestamp: u32,
        #[allow(missing_docs)]
        pub duration: u32,
        #[allow(missing_docs)]
        pub description: alloy::sol_types::private::String,
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
            alloy::sol_types::sol_data::Array<StrategyAndMultiplier>,
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Array<OperatorReward>,
            alloy::sol_types::sol_data::Uint<32>,
            alloy::sol_types::sol_data::Uint<32>,
            alloy::sol_types::sol_data::String,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Vec<
                <StrategyAndMultiplier as alloy::sol_types::SolType>::RustType,
            >,
            alloy::sol_types::private::Address,
            alloy::sol_types::private::Vec<<OperatorReward as alloy::sol_types::SolType>::RustType>,
            u32,
            u32,
            alloy::sol_types::private::String,
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
        impl ::core::convert::From<OperatorDirectedRewardsSubmission> for UnderlyingRustTuple<'_> {
            fn from(value: OperatorDirectedRewardsSubmission) -> Self {
                (
                    value.strategiesAndMultipliers,
                    value.token,
                    value.operatorRewards,
                    value.startTimestamp,
                    value.duration,
                    value.description,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OperatorDirectedRewardsSubmission {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    strategiesAndMultipliers: tuple.0,
                    token: tuple.1,
                    operatorRewards: tuple.2,
                    startTimestamp: tuple.3,
                    duration: tuple.4,
                    description: tuple.5,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for OperatorDirectedRewardsSubmission {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for OperatorDirectedRewardsSubmission {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        StrategyAndMultiplier,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.strategiesAndMultipliers,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.token,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        OperatorReward,
                    > as alloy_sol_types::SolType>::tokenize(&self.operatorRewards),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.startTimestamp),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.duration),
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.description,
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
        impl alloy_sol_types::SolType for OperatorDirectedRewardsSubmission {
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
        impl alloy_sol_types::SolStruct for OperatorDirectedRewardsSubmission {
            const NAME: &'static str = "OperatorDirectedRewardsSubmission";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "OperatorDirectedRewardsSubmission(StrategyAndMultiplier[] strategiesAndMultipliers,address token,OperatorReward[] operatorRewards,uint32 startTimestamp,uint32 duration,string description)",
                )
            }
            #[inline]
            fn eip712_components(
            ) -> alloy_sol_types::private::Vec<alloy_sol_types::private::Cow<'static, str>>
            {
                let mut components = alloy_sol_types::private::Vec::with_capacity(2);
                components.push(
                    <StrategyAndMultiplier as alloy_sol_types::SolStruct>::eip712_root_type(),
                );
                components.extend(
                    <StrategyAndMultiplier as alloy_sol_types::SolStruct>::eip712_components(),
                );
                components.push(<OperatorReward as alloy_sol_types::SolStruct>::eip712_root_type());
                components
                    .extend(<OperatorReward as alloy_sol_types::SolStruct>::eip712_components());
                components
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::Array<
                        StrategyAndMultiplier,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.strategiesAndMultipliers,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.token,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        OperatorReward,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.operatorRewards,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.startTimestamp,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.duration)
                        .0,
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::eip712_data_word(
                            &self.description,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for OperatorDirectedRewardsSubmission {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Array<
                        StrategyAndMultiplier,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.strategiesAndMultipliers,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.token,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        OperatorReward,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.operatorRewards,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.startTimestamp,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.duration,
                    )
                    + <alloy::sol_types::sol_data::String as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.description,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
                <alloy::sol_types::sol_data::Array<
                    StrategyAndMultiplier,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.strategiesAndMultipliers,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.token,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    OperatorReward,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.operatorRewards,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.startTimestamp,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.duration,
                    out,
                );
                <alloy::sol_types::sol_data::String as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.description,
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
    struct OperatorReward { address operator; uint256 amount; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OperatorReward {
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<OperatorReward> for UnderlyingRustTuple<'_> {
            fn from(value: OperatorReward) -> Self {
                (value.operator, value.amount)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OperatorReward {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    operator: tuple.0,
                    amount: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for OperatorReward {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for OperatorReward {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.operator,
                    ),
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.amount,
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
        impl alloy_sol_types::SolType for OperatorReward {
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
        impl alloy_sol_types::SolStruct for OperatorReward {
            const NAME: &'static str = "OperatorReward";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "OperatorReward(address operator,uint256 amount)",
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
                            &self.operator,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.amount)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for OperatorReward {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.operator,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.amount,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.operator,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.amount,
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
    struct RewardsSubmission { StrategyAndMultiplier[] strategiesAndMultipliers; address token; uint256 amount; uint32 startTimestamp; uint32 duration; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct RewardsSubmission {
        #[allow(missing_docs)]
        pub strategiesAndMultipliers: alloy::sol_types::private::Vec<
            <StrategyAndMultiplier as alloy::sol_types::SolType>::RustType,
        >,
        #[allow(missing_docs)]
        pub token: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub startTimestamp: u32,
        #[allow(missing_docs)]
        pub duration: u32,
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
            alloy::sol_types::sol_data::Array<StrategyAndMultiplier>,
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<32>,
            alloy::sol_types::sol_data::Uint<32>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Vec<
                <StrategyAndMultiplier as alloy::sol_types::SolType>::RustType,
            >,
            alloy::sol_types::private::Address,
            alloy::sol_types::private::primitives::aliases::U256,
            u32,
            u32,
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
        impl ::core::convert::From<RewardsSubmission> for UnderlyingRustTuple<'_> {
            fn from(value: RewardsSubmission) -> Self {
                (
                    value.strategiesAndMultipliers,
                    value.token,
                    value.amount,
                    value.startTimestamp,
                    value.duration,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for RewardsSubmission {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    strategiesAndMultipliers: tuple.0,
                    token: tuple.1,
                    amount: tuple.2,
                    startTimestamp: tuple.3,
                    duration: tuple.4,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for RewardsSubmission {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for RewardsSubmission {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        StrategyAndMultiplier,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.strategiesAndMultipliers,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.token,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.startTimestamp),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.duration),
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
        impl alloy_sol_types::SolType for RewardsSubmission {
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
        impl alloy_sol_types::SolStruct for RewardsSubmission {
            const NAME: &'static str = "RewardsSubmission";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "RewardsSubmission(StrategyAndMultiplier[] strategiesAndMultipliers,address token,uint256 amount,uint32 startTimestamp,uint32 duration)",
                )
            }
            #[inline]
            fn eip712_components(
            ) -> alloy_sol_types::private::Vec<alloy_sol_types::private::Cow<'static, str>>
            {
                let mut components = alloy_sol_types::private::Vec::with_capacity(1);
                components.push(
                    <StrategyAndMultiplier as alloy_sol_types::SolStruct>::eip712_root_type(),
                );
                components.extend(
                    <StrategyAndMultiplier as alloy_sol_types::SolStruct>::eip712_components(),
                );
                components
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::Array<
                        StrategyAndMultiplier,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.strategiesAndMultipliers,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.token,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.amount)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.startTimestamp,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.duration)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for RewardsSubmission {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Array<
                        StrategyAndMultiplier,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.strategiesAndMultipliers,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.token,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.amount,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.startTimestamp,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.duration,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
                <alloy::sol_types::sol_data::Array<
                    StrategyAndMultiplier,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.strategiesAndMultipliers,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.token,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.amount,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.startTimestamp,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.duration,
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
    struct StrategyAndMultiplier { address strategy; uint96 multiplier; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct StrategyAndMultiplier {
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
        impl ::core::convert::From<StrategyAndMultiplier> for UnderlyingRustTuple<'_> {
            fn from(value: StrategyAndMultiplier) -> Self {
                (value.strategy, value.multiplier)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for StrategyAndMultiplier {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    strategy: tuple.0,
                    multiplier: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for StrategyAndMultiplier {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for StrategyAndMultiplier {
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
        impl alloy_sol_types::SolType for StrategyAndMultiplier {
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
        impl alloy_sol_types::SolStruct for StrategyAndMultiplier {
            const NAME: &'static str = "StrategyAndMultiplier";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "StrategyAndMultiplier(address strategy,uint96 multiplier)",
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
        impl alloy_sol_types::EventTopic for StrategyAndMultiplier {
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
    /**Creates a new wrapper around an on-chain [`IRewardsCoordinatorTypes`](self) contract instance.

    See the [wrapper's documentation](`IRewardsCoordinatorTypesInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> IRewardsCoordinatorTypesInstance<T, P, N> {
        IRewardsCoordinatorTypesInstance::<T, P, N>::new(address, provider)
    }
    /**A [`IRewardsCoordinatorTypes`](self) instance.

    Contains type-safe methods for interacting with an on-chain instance of the
    [`IRewardsCoordinatorTypes`](self) contract located at a given `address`, using a given
    provider `P`.

    If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
    documentation on how to provide it), the `deploy` and `deploy_builder` methods can
    be used to deploy a new instance of the contract.

    See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct IRewardsCoordinatorTypesInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for IRewardsCoordinatorTypesInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("IRewardsCoordinatorTypesInstance")
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
        > IRewardsCoordinatorTypesInstance<T, P, N>
    {
        /**Creates a new wrapper around an on-chain [`IRewardsCoordinatorTypes`](self) contract instance.

        See the [wrapper's documentation](`IRewardsCoordinatorTypesInstance`) for more details.*/
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
    impl<T, P: ::core::clone::Clone, N> IRewardsCoordinatorTypesInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> IRewardsCoordinatorTypesInstance<T, P, N> {
            IRewardsCoordinatorTypesInstance {
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
        > IRewardsCoordinatorTypesInstance<T, P, N>
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
        > IRewardsCoordinatorTypesInstance<T, P, N>
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
library IHelloWorldServiceManager {
    struct Task {
        string name;
        uint32 taskCreatedBlock;
    }
}

library IRewardsCoordinatorTypes {
    struct OperatorDirectedRewardsSubmission {
        StrategyAndMultiplier[] strategiesAndMultipliers;
        address token;
        OperatorReward[] operatorRewards;
        uint32 startTimestamp;
        uint32 duration;
        string description;
    }
    struct OperatorReward {
        address operator;
        uint256 amount;
    }
    struct RewardsSubmission {
        StrategyAndMultiplier[] strategiesAndMultipliers;
        address token;
        uint256 amount;
        uint32 startTimestamp;
        uint32 duration;
    }
    struct StrategyAndMultiplier {
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

interface HelloWorldServiceManager {
    error DelayPeriodNotPassed();
    error OnlyRegistryCoordinator();
    error OnlyRewardsInitiator();
    error OnlyStakeRegistry();

    event Initialized(uint8 version);
    event NewTaskCreated(uint32 indexed taskIndex, IHelloWorldServiceManager.Task task);
    event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);
    event RewardsInitiatorUpdated(address prevRewardsInitiator, address newRewardsInitiator);
    event TaskResponded(uint32 indexed taskIndex, IHelloWorldServiceManager.Task task, address operator);

    constructor(address _avsDirectory, address _stakeRegistry, address _rewardsCoordinator, address _delegationManager, address _allocationManager, uint32 _maxResponseIntervalBlocks);

    function MAX_RESPONSE_INTERVAL_BLOCKS() external view returns (uint32);
    function addPendingAdmin(address admin) external;
    function allTaskHashes(uint32) external view returns (bytes32);
    function allTaskResponses(address, uint32) external view returns (bytes memory);
    function allocationManager() external view returns (address);
    function avsDirectory() external view returns (address);
    function createAVSRewardsSubmission(IRewardsCoordinatorTypes.RewardsSubmission[] memory rewardsSubmissions) external;
    function createNewTask(string memory name) external returns (IHelloWorldServiceManager.Task memory);
    function createOperatorDirectedAVSRewardsSubmission(IRewardsCoordinatorTypes.OperatorDirectedRewardsSubmission[] memory operatorDirectedRewardsSubmissions) external;
    function deregisterOperatorFromAVS(address operator) external;
    function deregisterOperatorFromOperatorSets(address operator, uint32[] memory operatorSetIds) external;
    function getOperatorRestakedStrategies(address _operator) external view returns (address[] memory);
    function getRestakeableStrategies() external view returns (address[] memory);
    function initialize(address initialOwner, address _rewardsInitiator) external;
    function latestTaskNum() external view returns (uint32);
    function owner() external view returns (address);
    function registerOperatorToAVS(address operator, ISignatureUtilsMixinTypes.SignatureWithSaltAndExpiry memory operatorSignature) external;
    function removeAdmin(address admin) external;
    function removeAppointee(address appointee, address target, bytes4 selector) external;
    function removePendingAdmin(address pendingAdmin) external;
    function renounceOwnership() external;
    function respondToTask(IHelloWorldServiceManager.Task memory task, uint32 referenceTaskIndex, bytes memory signature) external;
    function rewardsInitiator() external view returns (address);
    function setAVSRegistrar(address registrar) external;
    function setAppointee(address appointee, address target, bytes4 selector) external;
    function setClaimerFor(address claimer) external;
    function setRewardsInitiator(address newRewardsInitiator) external;
    function slashOperator(IHelloWorldServiceManager.Task memory task, uint32 referenceTaskIndex, address operator) external;
    function stakeRegistry() external view returns (address);
    function taskWasResponded(uint32) external view returns (bool);
    function transferOwnership(address newOwner) external;
    function updateAVSMetadataURI(string memory _metadataURI) external;
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "constructor",
    "inputs": [
      {
        "name": "_avsDirectory",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "_stakeRegistry",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "_rewardsCoordinator",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "_delegationManager",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "_allocationManager",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "_maxResponseIntervalBlocks",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "MAX_RESPONSE_INTERVAL_BLOCKS",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "addPendingAdmin",
    "inputs": [
      {
        "name": "admin",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "allTaskHashes",
    "inputs": [
      {
        "name": "",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "allTaskResponses",
    "inputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "allocationManager",
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
    "name": "avsDirectory",
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
    "name": "createAVSRewardsSubmission",
    "inputs": [
      {
        "name": "rewardsSubmissions",
        "type": "tuple[]",
        "internalType": "struct IRewardsCoordinatorTypes.RewardsSubmission[]",
        "components": [
          {
            "name": "strategiesAndMultipliers",
            "type": "tuple[]",
            "internalType": "struct IRewardsCoordinatorTypes.StrategyAndMultiplier[]",
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
          },
          {
            "name": "token",
            "type": "address",
            "internalType": "contract IERC20"
          },
          {
            "name": "amount",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "startTimestamp",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "duration",
            "type": "uint32",
            "internalType": "uint32"
          }
        ]
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "createNewTask",
    "inputs": [
      {
        "name": "name",
        "type": "string",
        "internalType": "string"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "tuple",
        "internalType": "struct IHelloWorldServiceManager.Task",
        "components": [
          {
            "name": "name",
            "type": "string",
            "internalType": "string"
          },
          {
            "name": "taskCreatedBlock",
            "type": "uint32",
            "internalType": "uint32"
          }
        ]
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "createOperatorDirectedAVSRewardsSubmission",
    "inputs": [
      {
        "name": "operatorDirectedRewardsSubmissions",
        "type": "tuple[]",
        "internalType": "struct IRewardsCoordinatorTypes.OperatorDirectedRewardsSubmission[]",
        "components": [
          {
            "name": "strategiesAndMultipliers",
            "type": "tuple[]",
            "internalType": "struct IRewardsCoordinatorTypes.StrategyAndMultiplier[]",
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
          },
          {
            "name": "token",
            "type": "address",
            "internalType": "contract IERC20"
          },
          {
            "name": "operatorRewards",
            "type": "tuple[]",
            "internalType": "struct IRewardsCoordinatorTypes.OperatorReward[]",
            "components": [
              {
                "name": "operator",
                "type": "address",
                "internalType": "address"
              },
              {
                "name": "amount",
                "type": "uint256",
                "internalType": "uint256"
              }
            ]
          },
          {
            "name": "startTimestamp",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "duration",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "description",
            "type": "string",
            "internalType": "string"
          }
        ]
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "deregisterOperatorFromAVS",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "deregisterOperatorFromOperatorSets",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "operatorSetIds",
        "type": "uint32[]",
        "internalType": "uint32[]"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "getOperatorRestakedStrategies",
    "inputs": [
      {
        "name": "_operator",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "address[]",
        "internalType": "address[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getRestakeableStrategies",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address[]",
        "internalType": "address[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "initialize",
    "inputs": [
      {
        "name": "initialOwner",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "_rewardsInitiator",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "latestTaskNum",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint32",
        "internalType": "uint32"
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
    "name": "registerOperatorToAVS",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "internalType": "address"
      },
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
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "removeAdmin",
    "inputs": [
      {
        "name": "admin",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "removeAppointee",
    "inputs": [
      {
        "name": "appointee",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "target",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "selector",
        "type": "bytes4",
        "internalType": "bytes4"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "removePendingAdmin",
    "inputs": [
      {
        "name": "pendingAdmin",
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
    "name": "respondToTask",
    "inputs": [
      {
        "name": "task",
        "type": "tuple",
        "internalType": "struct IHelloWorldServiceManager.Task",
        "components": [
          {
            "name": "name",
            "type": "string",
            "internalType": "string"
          },
          {
            "name": "taskCreatedBlock",
            "type": "uint32",
            "internalType": "uint32"
          }
        ]
      },
      {
        "name": "referenceTaskIndex",
        "type": "uint32",
        "internalType": "uint32"
      },
      {
        "name": "signature",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "rewardsInitiator",
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
    "name": "setAVSRegistrar",
    "inputs": [
      {
        "name": "registrar",
        "type": "address",
        "internalType": "contract IAVSRegistrar"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "setAppointee",
    "inputs": [
      {
        "name": "appointee",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "target",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "selector",
        "type": "bytes4",
        "internalType": "bytes4"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "setClaimerFor",
    "inputs": [
      {
        "name": "claimer",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "setRewardsInitiator",
    "inputs": [
      {
        "name": "newRewardsInitiator",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "slashOperator",
    "inputs": [
      {
        "name": "task",
        "type": "tuple",
        "internalType": "struct IHelloWorldServiceManager.Task",
        "components": [
          {
            "name": "name",
            "type": "string",
            "internalType": "string"
          },
          {
            "name": "taskCreatedBlock",
            "type": "uint32",
            "internalType": "uint32"
          }
        ]
      },
      {
        "name": "referenceTaskIndex",
        "type": "uint32",
        "internalType": "uint32"
      },
      {
        "name": "operator",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "stakeRegistry",
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
    "name": "taskWasResponded",
    "inputs": [
      {
        "name": "",
        "type": "uint32",
        "internalType": "uint32"
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
    "name": "updateAVSMetadataURI",
    "inputs": [
      {
        "name": "_metadataURI",
        "type": "string",
        "internalType": "string"
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
    "name": "NewTaskCreated",
    "inputs": [
      {
        "name": "taskIndex",
        "type": "uint32",
        "indexed": true,
        "internalType": "uint32"
      },
      {
        "name": "task",
        "type": "tuple",
        "indexed": false,
        "internalType": "struct IHelloWorldServiceManager.Task",
        "components": [
          {
            "name": "name",
            "type": "string",
            "internalType": "string"
          },
          {
            "name": "taskCreatedBlock",
            "type": "uint32",
            "internalType": "uint32"
          }
        ]
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
    "name": "RewardsInitiatorUpdated",
    "inputs": [
      {
        "name": "prevRewardsInitiator",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "newRewardsInitiator",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "TaskResponded",
    "inputs": [
      {
        "name": "taskIndex",
        "type": "uint32",
        "indexed": true,
        "internalType": "uint32"
      },
      {
        "name": "task",
        "type": "tuple",
        "indexed": false,
        "internalType": "struct IHelloWorldServiceManager.Task",
        "components": [
          {
            "name": "name",
            "type": "string",
            "internalType": "string"
          },
          {
            "name": "taskCreatedBlock",
            "type": "uint32",
            "internalType": "uint32"
          }
        ]
      },
      {
        "name": "operator",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "error",
    "name": "DelayPeriodNotPassed",
    "inputs": []
  },
  {
    "type": "error",
    "name": "OnlyRegistryCoordinator",
    "inputs": []
  },
  {
    "type": "error",
    "name": "OnlyRewardsInitiator",
    "inputs": []
  },
  {
    "type": "error",
    "name": "OnlyStakeRegistry",
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
pub mod HelloWorldServiceManager {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x610140346101eb57601f612bd938819003918201601f19168301916001600160401b038311848410176101ef5780849260c0946040528339810103126101eb5761004881610203565b9061005560208201610203565b61006160408301610203565b61006d60608401610203565b9160a061007c60808601610203565b9401519463ffffffff861686036101eb5760a05260805260e0526101005260c0525f5460ff8160081c166101965760ff8082160361015c575b50610120526040516129c1908161021882396080518181816104e20152818161082901528181610cc001528181610f090152818161132f0152818161197b0152611bb0015260a0518181816107a00152818161085b01528181610ce801526112eb015260c05181818161045601526105a9015260e0518181816101b7015281816109100152610b9901526101005181611c76015261012051818181610ea901526112570152f35b60ff90811916175f557f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb3847402498602060405160ff8152a15f6100b5565b60405162461bcd60e51b815260206004820152602760248201527f496e697469616c697a61626c653a20636f6e747261637420697320696e697469604482015266616c697a696e6760c81b6064820152608490fd5b5f80fd5b634e487b7160e01b5f52604160045260245ffd5b51906001600160a01b03821682036101eb5756fe60806040526004361015610011575f80fd5b5f5f3560e01c80631785f53c14611e865780631fdb0cfd14611ea8578063279432eb14611e865780632d89f6fc14611e5157806333cfb7b714611b835780633415a49c146115245780633bc28c8c14611500578063485cc9551461135e578063683048351461131a5780636b3aa72e146112d6578063715018a61461127b578063733d056c1461123b5780637b6c42361461120157806385edf874146111225780638b00ce7c146110ff5780638da5cb5b146110d75780639677de1014610db85780639926ee7d14610c245780639da16d8e14610c01578063a0169ddd14610b74578063a20b99bf146108d2578063a364f4da1461080d578063a98fb35514610786578063ba5508801461076c578063c1a8e2c5146106d7578063c20bab7f146105d8578063ca8aa7c714610593578063e481af9d146104bd578063f25f161014610424578063f2fde38b14610393578063fc299dee1461036a5763fce36c7d1461017a575f80fd5b34610257576020366003190112610257576004356001600160401b038111610366576101aa9036906004016120e4565b6101b592919261257c565b7f0000000000000000000000000000000000000000000000000000000000000000825b82811061030257506001600160a01b031690813b156102fe576040519363fce36c7d60e01b8552816024860160206004880152526044850160448360051b87010192828690609e19813603015b8383106102655788808b8181808c0381838f5af1801561025a576102465750f35b8161025091611fca565b6102575780f35b80fd5b6040513d84823e3d90fd5b9091929394956043198a82030186528635828112156102fa5760206001928582930190608063ffffffff6102e8826102ae6102a08780612617565b60a0885260a088019161264b565b95898060a01b036102c0898301611ed7565b168887015260408101356040870152836102dc60608301611f5c565b16606087015201611f5c565b16910152980196019493019190610225565b8980fd5b8280fd5b8061033761031e6020610318600195888b6126af565b01612603565b604061032b84888b6126af565b013590309033906126d1565b61036061034a602061031884888b6126af565b84604061035885898c6126af565b01359161271c565b016101d8565b5080fd5b50346102575780600319360112610257576065546040516001600160a01b039091168152602090f35b5034610257576020366003190112610257576103ad611ec1565b6103b561233c565b6001600160a01b038116156103d0576103cd90612534565b80f35b60405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b6064820152608490fd5b503461025757602036600319011261025757806004356001600160a01b038116908190036104ba5761045461233c565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690813b156104b85782916044839260405194859384926334f65bfd60e21b845230600485015260248401525af1801561025a576102465750f35b505b50fd5b50346102575780600319360112610257576040516302e0740360e31b815281816004817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa90811561025a578291610571575b506105268151516124a4565b915b8151805182101561055f57600191906001600160a01b039061054b908390612286565b5151166105588286612286565b5201610528565b6040518061056d8682611f6d565b0390f35b61058d91503d8084833e6105858183611fca565b810190612394565b5f61051a565b50346102575780600319360112610257576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b5034610257576040366003190112610257576105f2611ec1565b6105fa611f49565b9060018060a01b03168252609960205263ffffffff6040832091165f5260205260405f209060405191818154916106308361212b565b80865292600181169081156106ad575060011461066c575b61056d8561065881870382611fca565b604051918291602083526020830190612095565b815260208120939250905b808210610693575090915081016020016106588261056d610648565b919260018160209254838588010152019101909291610677565b86955061056d9693506020925061065894915060ff191682840152151560051b8201019293610648565b5034610257576040366003190112610257576106f1611ec1565b506024356001600160401b03811161036657366023820112156103665780600401356024602061072083612114565b61072d6040519182611fca565b838152019160051b8301019136831161076857602401905b828210610750578380f35b6020809161075d84611f5c565b815201910190610745565b8380fd5b50346102575761077b36611eeb565b5050506103cd61233c565b503461025757806107963661205a565b61079e61233c565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316803b156104b85760405163a98fb35560e01b81526020600482015291839183918290849082906107fc906024830190612095565b03925af1801561025a576102465750f35b503461025757602036600319011261025757610827611ec1565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031633036108c35781907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690813b156104b8576040516351b27a6d60e11b81526001600160a01b0390911660048201529082908290602490829084905af1801561025a576102465750f35b6346bf228160e01b8252600482fd5b5034610257576020366003190112610257576004356001600160401b038111610366576109039036906004016120e4565b61090e92919261257c565b7f0000000000000000000000000000000000000000000000000000000000000000825b828110610abf5750919283926001600160a01b031690813b15610aba57935060405191634e5cd2fd60e11b835280604484013060048601526040602486015252606483019060648160051b850101958092869160be19813603015b8484106109ad578880898181808f0381838e5af1801561025a576102465750f35b878a036063190183528535818112156102fa578201996109de6109d08c80612617565b60c0845260c084019161264b565b9a6001600160a01b036109f360208301611ed7565b1660208301526020610a086040830183612617565b848f036040860152808f529d9091019c8c5b818110610a89575050506020610a798c9d600194610a6b8563ffffffff610a446060889901611f5c565b16606084015263ffffffff610a5b60808301611f5c565b16608084015260a0810190612163565b9160a0818503910152612194565b980194019401939295995061098c565b90919d60408f60019260208392858060a01b03610aa582611ed7565b16835201356020820152019f01929101610a1a565b505050fd5b909291829483955b610adf610ad585858561259f565b60408101906125c1565b9050871015610b2f57610af6610ad585858561259f565b881015610b1b576001916020610b13928a60061b010135906125f6565b960195610ac7565b634e487b7160e01b86526032600452602486fd5b600192949593919650610b6e90610b5881308a610b536020610318898d339561259f565b6126d1565b84610b696020610318868a8d61259f565b61271c565b01610931565b50346102575760203660031901126102575780610b8f611ec1565b610b9761233c565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690813b156104b85760405163a0169ddd60e01b81526001600160a01b0390911660048201529082908290602490829084905af1801561025a576102465750f35b503461025757602036600319011261025757610c1b611ec1565b506103cd61233c565b5034610d91576040366003190112610d9157610c3e611ec1565b602435906001600160401b038211610d915760606003198336030112610d915760405190606082018281106001600160401b03821117610da45760405282600401356001600160401b038111610d9157610c9e906004369186010161203c565b8252602082019160248401358352604460408201940135845260018060a01b037f0000000000000000000000000000000000000000000000000000000000000000163303610d95577f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690813b15610d91575f610d5993819560405197889687958694639926ee7d60e01b865260018060a01b0316600486015260406024860152516060604486015260a4850190612095565b9151606484015251608483015203925af18015610d8657610d78575080f35b610d8491505f90611fca565b005b6040513d5f823e3d90fd5b5f80fd5b6346bf228160e01b5f5260045ffd5b634e487b7160e01b5f52604160045260245ffd5b34610d91576060366003190112610d91576004356001600160401b038111610d915760406003198236030112610d9157610df0611f49565b906044356001600160a01b0381169190829003610d9157610e5663ffffffff604051602081019060208252610e3d81610e2f60408201886004016121b4565b03601f198101835282611fca565b519020941693845f52609860205260405f2054146121e9565b825f52609a60205260ff60405f20541661108757602490825f52609960205260405f20845f52602052610e95610e8f60405f205461212b565b156122ae565b0163ffffffff610ece610ea78361225b565b7f00000000000000000000000000000000000000000000000000000000000000009061226c565b1643111561103357610edf9061225b565b604051630955f2d960e41b81526004810183905263ffffffff9190911660248201526020816044817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa908115610d86575f91611001575b5015610fa2575f52609960205260405f20905f5260205260405f20610f66815461212b565b601f8111610f82575b50600e661cdb185cda195960ca1b019055005b610f9c90825f52601f60205f20910160051c81019061230d565b81610f6f565b60405162461bcd60e51b815260206004820152603160248201527f4f70657261746f7220776173206e6f742072656769737465726564207768656e604482015270081d185cdac81dd85cc818dc99585d1959607a1b6064820152608490fd5b90506020813d60201161102b575b8161101c60209383611fca565b81010312610d91575183610f41565b3d915061100f565b60405162461bcd60e51b815260206004820152602660248201527f5461736b20726573706f6e73652074696d6520686173206e6f742065787069726044820152651959081e595d60d21b6064820152608490fd5b60405162461bcd60e51b815260206004820152602260248201527f5461736b2068617320616c7265616479206265656e20726573706f6e64656420604482015261746f60f01b6064820152608490fd5b34610d91575f366003190112610d91576033546040516001600160a01b039091168152602090f35b34610d91575f366003190112610d9157602063ffffffff60975416604051908152f35b34610d91576111303661205a565b611138612323565b50611141612323565b90815263ffffffff43166020820152604051602081019061116681610e2f85856120b9565b51902063ffffffff609754165f52609860205260405f205560975490600163ffffffff8316807f58180a6a0403a63c2b5ce4b85d129d46a80d37851b2216bd0a98b59e7309b847604051806111bb87826120b9565b0390a2019163ffffffff83116111ed5763ffffffff61056d93169063ffffffff191617609755604051918291826120b9565b634e487b7160e01b5f52601160045260245ffd5b34610d91576020366003190112610d915763ffffffff61121f611f36565b165f52609a602052602060ff60405f2054166040519015158152f35b34610d91575f366003190112610d9157602060405163ffffffff7f0000000000000000000000000000000000000000000000000000000000000000168152f35b34610d91575f366003190112610d915761129361233c565b603380546001600160a01b031981169091555f906001600160a01b03167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e08280a3005b34610d91575f366003190112610d91576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b34610d91575f366003190112610d91576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b34610d91576040366003190112610d9157611377611ec1565b602435906001600160a01b0382168203610d91575f549160ff8360081c1615928380946114f3575b80156114dc575b156114805760ff1981166001175f558361146f575b5060ff5f5460081c1615611416576113d56113da92612534565b6124d6565b6113e057005b61ff00195f54165f557f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb3847402498602060405160018152a1005b60405162461bcd60e51b815260206004820152602b60248201527f496e697469616c697a61626c653a20636f6e7472616374206973206e6f74206960448201526a6e697469616c697a696e6760a81b6064820152608490fd5b61ffff1916610101175f55836113bb565b60405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b6064820152608490fd5b50303b1580156113a65750600160ff8216146113a6565b50600160ff82161061139f565b34610d91576020366003190112610d9157610d8461151c611ec1565b6113d561233c565b34610d91576060366003190112610d91576004356001600160401b038111610d915736819003600482016040600319830112610d9157611562611f49565b906044356001600160401b038111610d915761158290369060040161203c565b926115a863ffffffff604051602081019060208252610e3d81610e2f60408201896121b4565b602485019463ffffffff6115be610ea78861225b565b164311611b2f5782359160221901821215610d9157016004810135906001600160401b038211610d9157602401908036038213610d915761162c602760405183819460208301966602432b6363796160cd1b88528484013781015f838201520301601f198101835282611fca565b5190207f19457468657265756d205369676e6564204d6573736167653a0a3332000000005f52601c52603c5f2092805181019060608160208401930312610d915760208101516001600160401b038111610d915781019582603f88011215610d9157602087015161169c81612114565b976116aa604051998a611fca565b818952602080808b019360051b8301010190858211610d9157604001915b818310611b0f5750505060408201516001600160401b038111610d915782019280603f85011215610d915760208401519361170285612114565b946117106040519687611fca565b8086526020808088019260051b8401010191838311610d915760408101915b838310611aa557505050505060608201519063ffffffff8216809203610d915761175d63ffffffff9161225b565b1603611a49575f5b8651811015611932576001600160a01b036117808289612286565b51165f52609960205260405f20855f526020526117a3610e8f60405f205461212b565b6117ad8184612286565b516001600160a01b036117c0838a612286565b51165f52609960205260405f20865f5260205260405f20908051906001600160401b038211610da4576117f3835461212b565b601f81116118f7575b50602090601f8311600114611890576001949392915f9183611885575b50505f19600383901b1c191690841b1790555b857f8eb2d2fcccf5801e10ff58cd73e8781ba923122963789378771f03c1148b023e838060a01b0361185e848c612286565b5116604051809160408252611876604083018b6121b4565b9060208301520390a201611765565b015190508b80611819565b90601f19831691845f52815f20925f5b8181106118df5750916001969594929183889593106118c7575b505050811b01905561182c565b01515f1960f88460031b161c191690558b80806118ba565b929360206001819287860151815501950193016118a0565b61192290845f5260205f20601f850160051c81019160208610611928575b601f0160051c019061230d565b8a6117fc565b9091508190611915565b61197760208388885f52609a835260405f20600160ff198254161790556040519384928392630b135d3f60e11b84526004840152604060248401526044830190612095565b03817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa908115610d86575f91611a06575b506001600160e01b031916630b135d3f60e11b036119cd57005b60405162461bcd60e51b8152602060048201526011602482015270496e76616c6964207369676e617475726560781b6044820152606490fd5b90506020813d602011611a41575b81611a2160209383611fca565b81010312610d9157516001600160e01b031981168103610d9157816119b3565b3d9150611a14565b60405162461bcd60e51b815260206004820152602e60248201527f5265666572656e636520626c6f636b206d757374206d61746368207461736b2060448201526d6372656174696f6e20626c6f636b60901b6064820152608490fd5b82516001600160401b038111610d915760209083010185603f82011215610d9157602081015191611ad583611feb565b611ae26040519182611fca565b8381526040838501018810610d91575f602085819660408397018386015e8301015281520192019161172f565b82516001600160a01b0381168103610d91578152602092830192016116c8565b60405162461bcd60e51b815260206004820152602660248201527f5461736b20726573706f6e73652074696d652068617320616c726561647920656044820152651e1c1a5c995960d21b6064820152608490fd5b34610d91576020366003190112610d9157611b9c611ec1565b6040516302e0740360e31b81525f816004817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa908115610d86575f91611e37575b509081515190611bf782612114565b90611c056040519283611fca565b828252611c1183612114565b602083019490601f19013686375f5b848110611e0957505060408051639004134760e01b81526001600160a01b0390921660048301526024820152815160448201819052909384916064830191905f5b818110611de757505f939283900391508290507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa928315610d86575f93611d55575b505f5f5b838110611d295750611cc3906124a4565b925f905f5b848110611cdd576040518061056d8882611f6d565b611ce78183612286565b51611cf5575b600101611cc8565b91600190611d21906001600160a01b03611d0f8688612286565b5116611d1b828a612286565b52612496565b929050611ced565b611d338186612286565b51611d41575b600101611cb2565b90611d4d600191612496565b919050611d39565b9092503d805f833e611d678183611fca565b810190602081830312610d91578051906001600160401b038211610d9157019080601f83011215610d91578151611d9d81612114565b92611dab6040519485611fca565b81845260208085019260051b820101928311610d9157602001905b828210611dd7575050509183611cae565b8151815260209182019101611dc6565b82516001600160a01b0316845287945060209384019390920191600101611c61565b600190818060a09895981b03611e20828551612286565b515116611e2d8287612286565b5201949194611c20565b611e4b91503d805f833e6105858183611fca565b82611be8565b34610d91576020366003190112610d915763ffffffff611e6f611f36565b165f526098602052602060405f2054604051908152f35b34610d91576020366003190112610d9157611e9f611ec1565b50610d8461233c565b34610d9157611eb636611eeb565b505050610d8461233c565b600435906001600160a01b0382168203610d9157565b35906001600160a01b0382168203610d9157565b6060906003190112610d91576004356001600160a01b0381168103610d9157906024356001600160a01b0381168103610d9157906044356001600160e01b031981168103610d915790565b6004359063ffffffff82168203610d9157565b6024359063ffffffff82168203610d9157565b359063ffffffff82168203610d9157565b60206040818301928281528451809452019201905f5b818110611f905750505090565b82516001600160a01b0316845260209384019390920191600101611f83565b604081019081106001600160401b03821117610da457604052565b90601f801991011681019081106001600160401b03821117610da457604052565b6001600160401b038111610da457601f01601f191660200190565b92919261201282611feb565b916120206040519384611fca565b829481845281830111610d91578281602093845f960137010152565b9080601f83011215610d915781602061205793359101612006565b90565b6020600319820112610d9157600435906001600160401b038211610d915780602383011215610d915781602461205793600401359101612006565b805180835260209291819084018484015e5f828201840152601f01601f1916010190565b60208152604063ffffffff60206120da855184838701526060860190612095565b9401511691015290565b9181601f84011215610d91578235916001600160401b038311610d91576020808501948460051b010111610d9157565b6001600160401b038111610da45760051b60200190565b90600182811c92168015612159575b602083101461214557565b634e487b7160e01b5f52602260045260245ffd5b91607f169161213a565b9035601e1982360301811215610d915701602081359101916001600160401b038211610d91578136038313610d9157565b908060209392818452848401375f828201840152601f01601f1916010190565b90602063ffffffff6121e2826121db6121cd8780612163565b604088526040880191612194565b9501611f5c565b1691015290565b156121f057565b60405162461bcd60e51b815260206004820152603d60248201527f737570706c696564207461736b20646f6573206e6f74206d617463682074686560448201527f206f6e65207265636f7264656420696e2074686520636f6e74726163740000006064820152608490fd5b3563ffffffff81168103610d915790565b9063ffffffff8091169116019063ffffffff82116111ed57565b805182101561229a5760209160051b010190565b634e487b7160e01b5f52603260045260245ffd5b156122b557565b60405162461bcd60e51b815260206004820152602a60248201527f4f70657261746f722068617320616c726561647920726573706f6e64656420746044820152696f20746865207461736b60b01b6064820152608490fd5b818110612318575050565b5f815560010161230d565b6040519061233082611faf565b5f602083606081520152565b6033546001600160a01b0316330361235057565b606460405162461bcd60e51b815260206004820152602060248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152fd5b602081830312610d91578051906001600160401b038211610d91570190602082820312610d915760405191602083018381106001600160401b03821117610da4576040528051906001600160401b038211610d91570181601f82011215610d915780519061240182612114565b9261240f6040519485611fca565b82845260208085019360061b83010191818311610d9157602001925b82841061243b5750505050815290565b604084830312610d91576040519061245282611faf565b84516001600160a01b0381168103610d915782526020850151906bffffffffffffffffffffffff82168203610d91578260209283604095015281520193019261242b565b5f1981146111ed5760010190565b906124ae82612114565b6124bb6040519182611fca565b82815280926124cc601f1991612114565b0190602036910137565b606554604080516001600160a01b038084168252841660208201529192917fe11cddf1816a43318ca175bbc52cd0185436e9cbead7c83acc54a73e461717e39190a16001600160a01b03166001600160a01b03199190911617606555565b603380546001600160a01b039283166001600160a01b0319821681179092559091167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e05f80a3565b6065546001600160a01b0316330361259057565b638e79fdb560e01b5f5260045ffd5b919081101561229a5760051b8101359060be1981360301821215610d91570190565b903590601e1981360301821215610d9157018035906001600160401b038211610d9157602001918160061b36038313610d9157565b919082018092116111ed57565b356001600160a01b0381168103610d915790565b9035601e1982360301811215610d915701602081359101916001600160401b038211610d91578160061b36038313610d9157565b916020908281520191905f905b8082106126655750505090565b909192833560018060a01b038116809103610d915781526020840135906bffffffffffffffffffffffff8216809203610d9157604081600193602083940152019401920190612658565b919081101561229a5760051b81013590609e1981360301821215610d91570190565b6040516323b872dd60e01b60208201526001600160a01b03928316602482015292909116604483015260648083019390935291815261271a91612715608483611fca565b6127da565b565b604051636eb1769f60e11b81523060048201526001600160a01b0383166024820152602081806044810103816001600160a01b0386165afa908115610d86575f916127a6575b5061271a93612770916125f6565b60405163095ea7b360e01b60208201526001600160a01b0390931660248401526044808401919091528252612715606483611fca565b90506020813d6020116127d2575b816127c160209383611fca565b81010312610d91575161271a612762565b3d91506127b4565b906128599160018060a01b03165f80604051936127f8604086611fca565b602085527f5361666545524332303a206c6f772d6c6576656c2063616c6c206661696c6564602086015260208151910182855af13d156128ea573d9161283d83611feb565b9261284b6040519485611fca565b83523d5f602085013e6128ee565b80519081159182156128c7575b50501561286f57565b60405162461bcd60e51b815260206004820152602a60248201527f5361666545524332303a204552433230206f7065726174696f6e20646964206e6044820152691bdd081cdd58d8d9595960b21b6064820152608490fd5b8192509060209181010312610d9157602001518015158103610d91575f80612866565b6060915b919290156129505750815115612902575090565b3b1561290b5790565b60405162461bcd60e51b815260206004820152601d60248201527f416464726573733a2063616c6c20746f206e6f6e2d636f6e74726163740000006044820152606490fd5b8251909150156129635750805190602001fd5b60405162461bcd60e51b815260206004820152908190612987906024830190612095565b0390fdfea26469706673582212203b546342ef21a7255d0d7b7411818c65d38edbcdfc1e788629169d92e5ba18bc64736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"a\x01@4a\x01\xEBW`\x1Fa+\xD98\x81\x90\x03\x91\x82\x01`\x1F\x19\x16\x83\x01\x91`\x01`\x01`@\x1B\x03\x83\x11\x84\x84\x10\x17a\x01\xEFW\x80\x84\x92`\xC0\x94`@R\x839\x81\x01\x03\x12a\x01\xEBWa\0H\x81a\x02\x03V[\x90a\0U` \x82\x01a\x02\x03V[a\0a`@\x83\x01a\x02\x03V[a\0m``\x84\x01a\x02\x03V[\x91`\xA0a\0|`\x80\x86\x01a\x02\x03V[\x94\x01Q\x94c\xFF\xFF\xFF\xFF\x86\x16\x86\x03a\x01\xEBW`\xA0R`\x80R`\xE0Ra\x01\0R`\xC0R_T`\xFF\x81`\x08\x1C\x16a\x01\x96W`\xFF\x80\x82\x16\x03a\x01\\W[Pa\x01 R`@Qa)\xC1\x90\x81a\x02\x18\x829`\x80Q\x81\x81\x81a\x04\xE2\x01R\x81\x81a\x08)\x01R\x81\x81a\x0C\xC0\x01R\x81\x81a\x0F\t\x01R\x81\x81a\x13/\x01R\x81\x81a\x19{\x01Ra\x1B\xB0\x01R`\xA0Q\x81\x81\x81a\x07\xA0\x01R\x81\x81a\x08[\x01R\x81\x81a\x0C\xE8\x01Ra\x12\xEB\x01R`\xC0Q\x81\x81\x81a\x04V\x01Ra\x05\xA9\x01R`\xE0Q\x81\x81\x81a\x01\xB7\x01R\x81\x81a\t\x10\x01Ra\x0B\x99\x01Ra\x01\0Q\x81a\x1Cv\x01Ra\x01 Q\x81\x81\x81a\x0E\xA9\x01Ra\x12W\x01R\xF3[`\xFF\x90\x81\x19\x16\x17_U\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98` `@Q`\xFF\x81R\xA1_a\0\xB5V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FInitializable: contract is initi`D\x82\x01Rfalizing`\xC8\x1B`d\x82\x01R`\x84\x90\xFD[_\x80\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[Q\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x01\xEBWV\xFE`\x80`@R`\x046\x10\x15a\0\x11W_\x80\xFD[__5`\xE0\x1C\x80c\x17\x85\xF5<\x14a\x1E\x86W\x80c\x1F\xDB\x0C\xFD\x14a\x1E\xA8W\x80c'\x942\xEB\x14a\x1E\x86W\x80c-\x89\xF6\xFC\x14a\x1EQW\x80c3\xCF\xB7\xB7\x14a\x1B\x83W\x80c4\x15\xA4\x9C\x14a\x15$W\x80c;\xC2\x8C\x8C\x14a\x15\0W\x80cH\\\xC9U\x14a\x13^W\x80ch0H5\x14a\x13\x1AW\x80ck:\xA7.\x14a\x12\xD6W\x80cqP\x18\xA6\x14a\x12{W\x80cs=\x05l\x14a\x12;W\x80c{lB6\x14a\x12\x01W\x80c\x85\xED\xF8t\x14a\x11\"W\x80c\x8B\0\xCE|\x14a\x10\xFFW\x80c\x8D\xA5\xCB[\x14a\x10\xD7W\x80c\x96w\xDE\x10\x14a\r\xB8W\x80c\x99&\xEE}\x14a\x0C$W\x80c\x9D\xA1m\x8E\x14a\x0C\x01W\x80c\xA0\x16\x9D\xDD\x14a\x0BtW\x80c\xA2\x0B\x99\xBF\x14a\x08\xD2W\x80c\xA3d\xF4\xDA\x14a\x08\rW\x80c\xA9\x8F\xB3U\x14a\x07\x86W\x80c\xBAU\x08\x80\x14a\x07lW\x80c\xC1\xA8\xE2\xC5\x14a\x06\xD7W\x80c\xC2\x0B\xAB\x7F\x14a\x05\xD8W\x80c\xCA\x8A\xA7\xC7\x14a\x05\x93W\x80c\xE4\x81\xAF\x9D\x14a\x04\xBDW\x80c\xF2_\x16\x10\x14a\x04$W\x80c\xF2\xFD\xE3\x8B\x14a\x03\x93W\x80c\xFC)\x9D\xEE\x14a\x03jWc\xFC\xE3l}\x14a\x01zW_\x80\xFD[4a\x02WW` 6`\x03\x19\x01\x12a\x02WW`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x03fWa\x01\xAA\x906\x90`\x04\x01a \xE4V[a\x01\xB5\x92\x91\x92a%|V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82[\x82\x81\x10a\x03\x02WP`\x01`\x01`\xA0\x1B\x03\x16\x90\x81;\x15a\x02\xFEW`@Q\x93c\xFC\xE3l}`\xE0\x1B\x85R\x81`$\x86\x01` `\x04\x88\x01RR`D\x85\x01`D\x83`\x05\x1B\x87\x01\x01\x92\x82\x86\x90`\x9E\x19\x816\x03\x01[\x83\x83\x10a\x02eW\x88\x80\x8B\x81\x81\x80\x8C\x03\x81\x83\x8FZ\xF1\x80\x15a\x02ZWa\x02FWP\xF3[\x81a\x02P\x91a\x1F\xCAV[a\x02WW\x80\xF3[\x80\xFD[`@Q=\x84\x82>=\x90\xFD[\x90\x91\x92\x93\x94\x95`C\x19\x8A\x82\x03\x01\x86R\x865\x82\x81\x12\x15a\x02\xFAW` `\x01\x92\x85\x82\x93\x01\x90`\x80c\xFF\xFF\xFF\xFFa\x02\xE8\x82a\x02\xAEa\x02\xA0\x87\x80a&\x17V[`\xA0\x88R`\xA0\x88\x01\x91a&KV[\x95\x89\x80`\xA0\x1B\x03a\x02\xC0\x89\x83\x01a\x1E\xD7V[\x16\x88\x87\x01R`@\x81\x015`@\x87\x01R\x83a\x02\xDC``\x83\x01a\x1F\\V[\x16``\x87\x01R\x01a\x1F\\V[\x16\x91\x01R\x98\x01\x96\x01\x94\x93\x01\x91\x90a\x02%V[\x89\x80\xFD[\x82\x80\xFD[\x80a\x037a\x03\x1E` a\x03\x18`\x01\x95\x88\x8Ba&\xAFV[\x01a&\x03V[`@a\x03+\x84\x88\x8Ba&\xAFV[\x015\x900\x903\x90a&\xD1V[a\x03`a\x03J` a\x03\x18\x84\x88\x8Ba&\xAFV[\x84`@a\x03X\x85\x89\x8Ca&\xAFV[\x015\x91a'\x1CV[\x01a\x01\xD8V[P\x80\xFD[P4a\x02WW\x80`\x03\x196\x01\x12a\x02WW`eT`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[P4a\x02WW` 6`\x03\x19\x01\x12a\x02WWa\x03\xADa\x1E\xC1V[a\x03\xB5a#<V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a\x03\xD0Wa\x03\xCD\x90a%4V[\x80\xF3[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x90\xFD[P4a\x02WW` 6`\x03\x19\x01\x12a\x02WW\x80`\x045`\x01`\x01`\xA0\x1B\x03\x81\x16\x90\x81\x90\x03a\x04\xBAWa\x04Ta#<V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90\x81;\x15a\x04\xB8W\x82\x91`D\x83\x92`@Q\x94\x85\x93\x84\x92c4\xF6[\xFD`\xE2\x1B\x84R0`\x04\x85\x01R`$\x84\x01RZ\xF1\x80\x15a\x02ZWa\x02FWP\xF3[P[P\xFD[P4a\x02WW\x80`\x03\x196\x01\x12a\x02WW`@Qc\x02\xE0t\x03`\xE3\x1B\x81R\x81\x81`\x04\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x02ZW\x82\x91a\x05qW[Pa\x05&\x81QQa$\xA4V[\x91[\x81Q\x80Q\x82\x10\x15a\x05_W`\x01\x91\x90`\x01`\x01`\xA0\x1B\x03\x90a\x05K\x90\x83\x90a\"\x86V[QQ\x16a\x05X\x82\x86a\"\x86V[R\x01a\x05(V[`@Q\x80a\x05m\x86\x82a\x1FmV[\x03\x90\xF3[a\x05\x8D\x91P=\x80\x84\x83>a\x05\x85\x81\x83a\x1F\xCAV[\x81\x01\x90a#\x94V[_a\x05\x1AV[P4a\x02WW\x80`\x03\x196\x01\x12a\x02WW`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[P4a\x02WW`@6`\x03\x19\x01\x12a\x02WWa\x05\xF2a\x1E\xC1V[a\x05\xFAa\x1FIV[\x90`\x01\x80`\xA0\x1B\x03\x16\x82R`\x99` Rc\xFF\xFF\xFF\xFF`@\x83 \x91\x16_R` R`@_ \x90`@Q\x91\x81\x81T\x91a\x060\x83a!+V[\x80\x86R\x92`\x01\x81\x16\x90\x81\x15a\x06\xADWP`\x01\x14a\x06lW[a\x05m\x85a\x06X\x81\x87\x03\x82a\x1F\xCAV[`@Q\x91\x82\x91` \x83R` \x83\x01\x90a \x95V[\x81R` \x81 \x93\x92P\x90[\x80\x82\x10a\x06\x93WP\x90\x91P\x81\x01` \x01a\x06X\x82a\x05ma\x06HV[\x91\x92`\x01\x81` \x92T\x83\x85\x88\x01\x01R\x01\x91\x01\x90\x92\x91a\x06wV[\x86\x95Pa\x05m\x96\x93P` \x92Pa\x06X\x94\x91P`\xFF\x19\x16\x82\x84\x01R\x15\x15`\x05\x1B\x82\x01\x01\x92\x93a\x06HV[P4a\x02WW`@6`\x03\x19\x01\x12a\x02WWa\x06\xF1a\x1E\xC1V[P`$5`\x01`\x01`@\x1B\x03\x81\x11a\x03fW6`#\x82\x01\x12\x15a\x03fW\x80`\x04\x015`$` a\x07 \x83a!\x14V[a\x07-`@Q\x91\x82a\x1F\xCAV[\x83\x81R\x01\x91`\x05\x1B\x83\x01\x01\x916\x83\x11a\x07hW`$\x01\x90[\x82\x82\x10a\x07PW\x83\x80\xF3[` \x80\x91a\x07]\x84a\x1F\\V[\x81R\x01\x91\x01\x90a\x07EV[\x83\x80\xFD[P4a\x02WWa\x07{6a\x1E\xEBV[PPPa\x03\xCDa#<V[P4a\x02WW\x80a\x07\x966a ZV[a\x07\x9Ea#<V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x04\xB8W`@Qc\xA9\x8F\xB3U`\xE0\x1B\x81R` `\x04\x82\x01R\x91\x83\x91\x83\x91\x82\x90\x84\x90\x82\x90a\x07\xFC\x90`$\x83\x01\x90a \x95V[\x03\x92Z\xF1\x80\x15a\x02ZWa\x02FWP\xF3[P4a\x02WW` 6`\x03\x19\x01\x12a\x02WWa\x08'a\x1E\xC1V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x163\x03a\x08\xC3W\x81\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90\x81;\x15a\x04\xB8W`@QcQ\xB2zm`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R\x90\x82\x90\x82\x90`$\x90\x82\x90\x84\x90Z\xF1\x80\x15a\x02ZWa\x02FWP\xF3[cF\xBF\"\x81`\xE0\x1B\x82R`\x04\x82\xFD[P4a\x02WW` 6`\x03\x19\x01\x12a\x02WW`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x03fWa\t\x03\x906\x90`\x04\x01a \xE4V[a\t\x0E\x92\x91\x92a%|V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82[\x82\x81\x10a\n\xBFWP\x91\x92\x83\x92`\x01`\x01`\xA0\x1B\x03\x16\x90\x81;\x15a\n\xBAW\x93P`@Q\x91cN\\\xD2\xFD`\xE1\x1B\x83R\x80`D\x84\x010`\x04\x86\x01R`@`$\x86\x01RR`d\x83\x01\x90`d\x81`\x05\x1B\x85\x01\x01\x95\x80\x92\x86\x91`\xBE\x19\x816\x03\x01[\x84\x84\x10a\t\xADW\x88\x80\x89\x81\x81\x80\x8F\x03\x81\x83\x8EZ\xF1\x80\x15a\x02ZWa\x02FWP\xF3[\x87\x8A\x03`c\x19\x01\x83R\x855\x81\x81\x12\x15a\x02\xFAW\x82\x01\x99a\t\xDEa\t\xD0\x8C\x80a&\x17V[`\xC0\x84R`\xC0\x84\x01\x91a&KV[\x9A`\x01`\x01`\xA0\x1B\x03a\t\xF3` \x83\x01a\x1E\xD7V[\x16` \x83\x01R` a\n\x08`@\x83\x01\x83a&\x17V[\x84\x8F\x03`@\x86\x01R\x80\x8FR\x9D\x90\x91\x01\x9C\x8C[\x81\x81\x10a\n\x89WPPP` a\ny\x8C\x9D`\x01\x94a\nk\x85c\xFF\xFF\xFF\xFFa\nD``\x88\x99\x01a\x1F\\V[\x16``\x84\x01Rc\xFF\xFF\xFF\xFFa\n[`\x80\x83\x01a\x1F\\V[\x16`\x80\x84\x01R`\xA0\x81\x01\x90a!cV[\x91`\xA0\x81\x85\x03\x91\x01Ra!\x94V[\x98\x01\x94\x01\x94\x01\x93\x92\x95\x99Pa\t\x8CV[\x90\x91\x9D`@\x8F`\x01\x92` \x83\x92\x85\x80`\xA0\x1B\x03a\n\xA5\x82a\x1E\xD7V[\x16\x83R\x015` \x82\x01R\x01\x9F\x01\x92\x91\x01a\n\x1AV[PPP\xFD[\x90\x92\x91\x82\x94\x83\x95[a\n\xDFa\n\xD5\x85\x85\x85a%\x9FV[`@\x81\x01\x90a%\xC1V[\x90P\x87\x10\x15a\x0B/Wa\n\xF6a\n\xD5\x85\x85\x85a%\x9FV[\x88\x10\x15a\x0B\x1BW`\x01\x91` a\x0B\x13\x92\x8A`\x06\x1B\x01\x015\x90a%\xF6V[\x96\x01\x95a\n\xC7V[cNH{q`\xE0\x1B\x86R`2`\x04R`$\x86\xFD[`\x01\x92\x94\x95\x93\x91\x96Pa\x0Bn\x90a\x0BX\x810\x8Aa\x0BS` a\x03\x18\x89\x8D3\x95a%\x9FV[a&\xD1V[\x84a\x0Bi` a\x03\x18\x86\x8A\x8Da%\x9FV[a'\x1CV[\x01a\t1V[P4a\x02WW` 6`\x03\x19\x01\x12a\x02WW\x80a\x0B\x8Fa\x1E\xC1V[a\x0B\x97a#<V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90\x81;\x15a\x04\xB8W`@Qc\xA0\x16\x9D\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R\x90\x82\x90\x82\x90`$\x90\x82\x90\x84\x90Z\xF1\x80\x15a\x02ZWa\x02FWP\xF3[P4a\x02WW` 6`\x03\x19\x01\x12a\x02WWa\x0C\x1Ba\x1E\xC1V[Pa\x03\xCDa#<V[P4a\r\x91W`@6`\x03\x19\x01\x12a\r\x91Wa\x0C>a\x1E\xC1V[`$5\x90`\x01`\x01`@\x1B\x03\x82\x11a\r\x91W```\x03\x19\x836\x03\x01\x12a\r\x91W`@Q\x90``\x82\x01\x82\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\r\xA4W`@R\x82`\x04\x015`\x01`\x01`@\x1B\x03\x81\x11a\r\x91Wa\x0C\x9E\x90`\x046\x91\x86\x01\x01a <V[\x82R` \x82\x01\x91`$\x84\x015\x83R`D`@\x82\x01\x94\x015\x84R`\x01\x80`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x163\x03a\r\x95W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90\x81;\x15a\r\x91W_a\rY\x93\x81\x95`@Q\x97\x88\x96\x87\x95\x86\x94c\x99&\xEE}`\xE0\x1B\x86R`\x01\x80`\xA0\x1B\x03\x16`\x04\x86\x01R`@`$\x86\x01RQ```D\x86\x01R`\xA4\x85\x01\x90a \x95V[\x91Q`d\x84\x01RQ`\x84\x83\x01R\x03\x92Z\xF1\x80\x15a\r\x86Wa\rxWP\x80\xF3[a\r\x84\x91P_\x90a\x1F\xCAV[\0[`@Q=_\x82>=\x90\xFD[_\x80\xFD[cF\xBF\"\x81`\xE0\x1B_R`\x04_\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[4a\r\x91W``6`\x03\x19\x01\x12a\r\x91W`\x045`\x01`\x01`@\x1B\x03\x81\x11a\r\x91W`@`\x03\x19\x826\x03\x01\x12a\r\x91Wa\r\xF0a\x1FIV[\x90`D5`\x01`\x01`\xA0\x1B\x03\x81\x16\x91\x90\x82\x90\x03a\r\x91Wa\x0EVc\xFF\xFF\xFF\xFF`@Q` \x81\x01\x90` \x82Ra\x0E=\x81a\x0E/`@\x82\x01\x88`\x04\x01a!\xB4V[\x03`\x1F\x19\x81\x01\x83R\x82a\x1F\xCAV[Q\x90 \x94\x16\x93\x84_R`\x98` R`@_ T\x14a!\xE9V[\x82_R`\x9A` R`\xFF`@_ T\x16a\x10\x87W`$\x90\x82_R`\x99` R`@_ \x84_R` Ra\x0E\x95a\x0E\x8F`@_ Ta!+V[\x15a\"\xAEV[\x01c\xFF\xFF\xFF\xFFa\x0E\xCEa\x0E\xA7\x83a\"[V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90a\"lV[\x16C\x11\x15a\x103Wa\x0E\xDF\x90a\"[V[`@Qc\tU\xF2\xD9`\xE4\x1B\x81R`\x04\x81\x01\x83\x90Rc\xFF\xFF\xFF\xFF\x91\x90\x91\x16`$\x82\x01R` \x81`D\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\r\x86W_\x91a\x10\x01W[P\x15a\x0F\xA2W_R`\x99` R`@_ \x90_R` R`@_ a\x0Ff\x81Ta!+V[`\x1F\x81\x11a\x0F\x82W[P`\x0Ef\x1C\xDB\x18\\\xDA\x19Y`\xCA\x1B\x01\x90U\0[a\x0F\x9C\x90\x82_R`\x1F` _ \x91\x01`\x05\x1C\x81\x01\x90a#\rV[\x81a\x0FoV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`1`$\x82\x01R\x7FOperator was not registered when`D\x82\x01Rp\x08\x1D\x18\\\xDA\xC8\x1D\xD8\\\xC8\x18\xDC\x99X]\x19Y`z\x1B`d\x82\x01R`\x84\x90\xFD[\x90P` \x81=` \x11a\x10+W[\x81a\x10\x1C` \x93\x83a\x1F\xCAV[\x81\x01\x03\x12a\r\x91WQ\x83a\x0FAV[=\x91Pa\x10\x0FV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FTask response time has not expir`D\x82\x01Re\x19Y\x08\x1EY]`\xD2\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FTask has already been responded `D\x82\x01Rato`\xF0\x1B`d\x82\x01R`\x84\x90\xFD[4a\r\x91W_6`\x03\x19\x01\x12a\r\x91W`3T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\r\x91W_6`\x03\x19\x01\x12a\r\x91W` c\xFF\xFF\xFF\xFF`\x97T\x16`@Q\x90\x81R\xF3[4a\r\x91Wa\x1106a ZV[a\x118a##V[Pa\x11Aa##V[\x90\x81Rc\xFF\xFF\xFF\xFFC\x16` \x82\x01R`@Q` \x81\x01\x90a\x11f\x81a\x0E/\x85\x85a \xB9V[Q\x90 c\xFF\xFF\xFF\xFF`\x97T\x16_R`\x98` R`@_ U`\x97T\x90`\x01c\xFF\xFF\xFF\xFF\x83\x16\x80\x7FX\x18\nj\x04\x03\xA6<+\\\xE4\xB8]\x12\x9DF\xA8\r7\x85\x1B\"\x16\xBD\n\x98\xB5\x9Es\t\xB8G`@Q\x80a\x11\xBB\x87\x82a \xB9V[\x03\x90\xA2\x01\x91c\xFF\xFF\xFF\xFF\x83\x11a\x11\xEDWc\xFF\xFF\xFF\xFFa\x05m\x93\x16\x90c\xFF\xFF\xFF\xFF\x19\x16\x17`\x97U`@Q\x91\x82\x91\x82a \xB9V[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[4a\r\x91W` 6`\x03\x19\x01\x12a\r\x91Wc\xFF\xFF\xFF\xFFa\x12\x1Fa\x1F6V[\x16_R`\x9A` R` `\xFF`@_ T\x16`@Q\x90\x15\x15\x81R\xF3[4a\r\x91W_6`\x03\x19\x01\x12a\r\x91W` `@Qc\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R\xF3[4a\r\x91W_6`\x03\x19\x01\x12a\r\x91Wa\x12\x93a#<V[`3\x80T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16\x90\x91U_\x90`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x82\x80\xA3\0[4a\r\x91W_6`\x03\x19\x01\x12a\r\x91W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\r\x91W_6`\x03\x19\x01\x12a\r\x91W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\r\x91W`@6`\x03\x19\x01\x12a\r\x91Wa\x13wa\x1E\xC1V[`$5\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\r\x91W_T\x91`\xFF\x83`\x08\x1C\x16\x15\x92\x83\x80\x94a\x14\xF3W[\x80\x15a\x14\xDCW[\x15a\x14\x80W`\xFF\x19\x81\x16`\x01\x17_U\x83a\x14oW[P`\xFF_T`\x08\x1C\x16\x15a\x14\x16Wa\x13\xD5a\x13\xDA\x92a%4V[a$\xD6V[a\x13\xE0W\0[a\xFF\0\x19_T\x16_U\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98` `@Q`\x01\x81R\xA1\0[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x90\xFD[a\xFF\xFF\x19\x16a\x01\x01\x17_U\x83a\x13\xBBV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x90\xFD[P0;\x15\x80\x15a\x13\xA6WP`\x01`\xFF\x82\x16\x14a\x13\xA6V[P`\x01`\xFF\x82\x16\x10a\x13\x9FV[4a\r\x91W` 6`\x03\x19\x01\x12a\r\x91Wa\r\x84a\x15\x1Ca\x1E\xC1V[a\x13\xD5a#<V[4a\r\x91W``6`\x03\x19\x01\x12a\r\x91W`\x045`\x01`\x01`@\x1B\x03\x81\x11a\r\x91W6\x81\x90\x03`\x04\x82\x01`@`\x03\x19\x83\x01\x12a\r\x91Wa\x15ba\x1FIV[\x90`D5`\x01`\x01`@\x1B\x03\x81\x11a\r\x91Wa\x15\x82\x906\x90`\x04\x01a <V[\x92a\x15\xA8c\xFF\xFF\xFF\xFF`@Q` \x81\x01\x90` \x82Ra\x0E=\x81a\x0E/`@\x82\x01\x89a!\xB4V[`$\x85\x01\x94c\xFF\xFF\xFF\xFFa\x15\xBEa\x0E\xA7\x88a\"[V[\x16C\x11a\x1B/W\x825\x91`\"\x19\x01\x82\x12\x15a\r\x91W\x01`\x04\x81\x015\x90`\x01`\x01`@\x1B\x03\x82\x11a\r\x91W`$\x01\x90\x806\x03\x82\x13a\r\x91Wa\x16,`'`@Q\x83\x81\x94` \x83\x01\x96f\x02C+ccya`\xCD\x1B\x88R\x84\x84\x017\x81\x01_\x83\x82\x01R\x03\x01`\x1F\x19\x81\x01\x83R\x82a\x1F\xCAV[Q\x90 \x7F\x19Ethereum Signed Message:\n32\0\0\0\0_R`\x1CR`<_ \x92\x80Q\x81\x01\x90``\x81` \x84\x01\x93\x03\x12a\r\x91W` \x81\x01Q`\x01`\x01`@\x1B\x03\x81\x11a\r\x91W\x81\x01\x95\x82`?\x88\x01\x12\x15a\r\x91W` \x87\x01Qa\x16\x9C\x81a!\x14V[\x97a\x16\xAA`@Q\x99\x8Aa\x1F\xCAV[\x81\x89R` \x80\x80\x8B\x01\x93`\x05\x1B\x83\x01\x01\x01\x90\x85\x82\x11a\r\x91W`@\x01\x91[\x81\x83\x10a\x1B\x0FWPPP`@\x82\x01Q`\x01`\x01`@\x1B\x03\x81\x11a\r\x91W\x82\x01\x92\x80`?\x85\x01\x12\x15a\r\x91W` \x84\x01Q\x93a\x17\x02\x85a!\x14V[\x94a\x17\x10`@Q\x96\x87a\x1F\xCAV[\x80\x86R` \x80\x80\x88\x01\x92`\x05\x1B\x84\x01\x01\x01\x91\x83\x83\x11a\r\x91W`@\x81\x01\x91[\x83\x83\x10a\x1A\xA5WPPPPP``\x82\x01Q\x90c\xFF\xFF\xFF\xFF\x82\x16\x80\x92\x03a\r\x91Wa\x17]c\xFF\xFF\xFF\xFF\x91a\"[V[\x16\x03a\x1AIW_[\x86Q\x81\x10\x15a\x192W`\x01`\x01`\xA0\x1B\x03a\x17\x80\x82\x89a\"\x86V[Q\x16_R`\x99` R`@_ \x85_R` Ra\x17\xA3a\x0E\x8F`@_ Ta!+V[a\x17\xAD\x81\x84a\"\x86V[Q`\x01`\x01`\xA0\x1B\x03a\x17\xC0\x83\x8Aa\"\x86V[Q\x16_R`\x99` R`@_ \x86_R` R`@_ \x90\x80Q\x90`\x01`\x01`@\x1B\x03\x82\x11a\r\xA4Wa\x17\xF3\x83Ta!+V[`\x1F\x81\x11a\x18\xF7W[P` \x90`\x1F\x83\x11`\x01\x14a\x18\x90W`\x01\x94\x93\x92\x91_\x91\x83a\x18\x85W[PP_\x19`\x03\x83\x90\x1B\x1C\x19\x16\x90\x84\x1B\x17\x90U[\x85\x7F\x8E\xB2\xD2\xFC\xCC\xF5\x80\x1E\x10\xFFX\xCDs\xE8x\x1B\xA9#\x12)cx\x93xw\x1F\x03\xC1\x14\x8B\x02>\x83\x80`\xA0\x1B\x03a\x18^\x84\x8Ca\"\x86V[Q\x16`@Q\x80\x91`@\x82Ra\x18v`@\x83\x01\x8Ba!\xB4V[\x90` \x83\x01R\x03\x90\xA2\x01a\x17eV[\x01Q\x90P\x8B\x80a\x18\x19V[\x90`\x1F\x19\x83\x16\x91\x84_R\x81_ \x92_[\x81\x81\x10a\x18\xDFWP\x91`\x01\x96\x95\x94\x92\x91\x83\x88\x95\x93\x10a\x18\xC7W[PPP\x81\x1B\x01\x90Ua\x18,V[\x01Q_\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U\x8B\x80\x80a\x18\xBAV[\x92\x93` `\x01\x81\x92\x87\x86\x01Q\x81U\x01\x95\x01\x93\x01a\x18\xA0V[a\x19\"\x90\x84_R` _ `\x1F\x85\x01`\x05\x1C\x81\x01\x91` \x86\x10a\x19(W[`\x1F\x01`\x05\x1C\x01\x90a#\rV[\x8Aa\x17\xFCV[\x90\x91P\x81\x90a\x19\x15V[a\x19w` \x83\x88\x88_R`\x9A\x83R`@_ `\x01`\xFF\x19\x82T\x16\x17\x90U`@Q\x93\x84\x92\x83\x92c\x0B\x13]?`\xE1\x1B\x84R`\x04\x84\x01R`@`$\x84\x01R`D\x83\x01\x90a \x95V[\x03\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\r\x86W_\x91a\x1A\x06W[P`\x01`\x01`\xE0\x1B\x03\x19\x16c\x0B\x13]?`\xE1\x1B\x03a\x19\xCDW\0[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01RpInvalid signature`x\x1B`D\x82\x01R`d\x90\xFD[\x90P` \x81=` \x11a\x1AAW[\x81a\x1A!` \x93\x83a\x1F\xCAV[\x81\x01\x03\x12a\r\x91WQ`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x03a\r\x91W\x81a\x19\xB3V[=\x91Pa\x1A\x14V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FReference block must match task `D\x82\x01Rmcreation block`\x90\x1B`d\x82\x01R`\x84\x90\xFD[\x82Q`\x01`\x01`@\x1B\x03\x81\x11a\r\x91W` \x90\x83\x01\x01\x85`?\x82\x01\x12\x15a\r\x91W` \x81\x01Q\x91a\x1A\xD5\x83a\x1F\xEBV[a\x1A\xE2`@Q\x91\x82a\x1F\xCAV[\x83\x81R`@\x83\x85\x01\x01\x88\x10a\r\x91W_` \x85\x81\x96`@\x83\x97\x01\x83\x86\x01^\x83\x01\x01R\x81R\x01\x92\x01\x91a\x17/V[\x82Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\r\x91W\x81R` \x92\x83\x01\x92\x01a\x16\xC8V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FTask response time has already e`D\x82\x01Re\x1E\x1C\x1A\\\x99Y`\xD2\x1B`d\x82\x01R`\x84\x90\xFD[4a\r\x91W` 6`\x03\x19\x01\x12a\r\x91Wa\x1B\x9Ca\x1E\xC1V[`@Qc\x02\xE0t\x03`\xE3\x1B\x81R_\x81`\x04\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\r\x86W_\x91a\x1E7W[P\x90\x81QQ\x90a\x1B\xF7\x82a!\x14V[\x90a\x1C\x05`@Q\x92\x83a\x1F\xCAV[\x82\x82Ra\x1C\x11\x83a!\x14V[` \x83\x01\x94\x90`\x1F\x19\x016\x867_[\x84\x81\x10a\x1E\tWPP`@\x80Qc\x90\x04\x13G`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`\x04\x83\x01R`$\x82\x01R\x81Q`D\x82\x01\x81\x90R\x90\x93\x84\x91`d\x83\x01\x91\x90_[\x81\x81\x10a\x1D\xE7WP_\x93\x92\x83\x90\x03\x91P\x82\x90P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x92\x83\x15a\r\x86W_\x93a\x1DUW[P__[\x83\x81\x10a\x1D)WPa\x1C\xC3\x90a$\xA4V[\x92_\x90_[\x84\x81\x10a\x1C\xDDW`@Q\x80a\x05m\x88\x82a\x1FmV[a\x1C\xE7\x81\x83a\"\x86V[Qa\x1C\xF5W[`\x01\x01a\x1C\xC8V[\x91`\x01\x90a\x1D!\x90`\x01`\x01`\xA0\x1B\x03a\x1D\x0F\x86\x88a\"\x86V[Q\x16a\x1D\x1B\x82\x8Aa\"\x86V[Ra$\x96V[\x92\x90Pa\x1C\xEDV[a\x1D3\x81\x86a\"\x86V[Qa\x1DAW[`\x01\x01a\x1C\xB2V[\x90a\x1DM`\x01\x91a$\x96V[\x91\x90Pa\x1D9V[\x90\x92P=\x80_\x83>a\x1Dg\x81\x83a\x1F\xCAV[\x81\x01\x90` \x81\x83\x03\x12a\r\x91W\x80Q\x90`\x01`\x01`@\x1B\x03\x82\x11a\r\x91W\x01\x90\x80`\x1F\x83\x01\x12\x15a\r\x91W\x81Qa\x1D\x9D\x81a!\x14V[\x92a\x1D\xAB`@Q\x94\x85a\x1F\xCAV[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\r\x91W` \x01\x90[\x82\x82\x10a\x1D\xD7WPPP\x91\x83a\x1C\xAEV[\x81Q\x81R` \x91\x82\x01\x91\x01a\x1D\xC6V[\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x84R\x87\x94P` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x1CaV[`\x01\x90\x81\x80`\xA0\x98\x95\x98\x1B\x03a\x1E \x82\x85Qa\"\x86V[QQ\x16a\x1E-\x82\x87a\"\x86V[R\x01\x94\x91\x94a\x1C V[a\x1EK\x91P=\x80_\x83>a\x05\x85\x81\x83a\x1F\xCAV[\x82a\x1B\xE8V[4a\r\x91W` 6`\x03\x19\x01\x12a\r\x91Wc\xFF\xFF\xFF\xFFa\x1Eoa\x1F6V[\x16_R`\x98` R` `@_ T`@Q\x90\x81R\xF3[4a\r\x91W` 6`\x03\x19\x01\x12a\r\x91Wa\x1E\x9Fa\x1E\xC1V[Pa\r\x84a#<V[4a\r\x91Wa\x1E\xB66a\x1E\xEBV[PPPa\r\x84a#<V[`\x045\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\r\x91WV[5\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\r\x91WV[``\x90`\x03\x19\x01\x12a\r\x91W`\x045`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\r\x91W\x90`$5`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\r\x91W\x90`D5`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x03a\r\x91W\x90V[`\x045\x90c\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\r\x91WV[`$5\x90c\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\r\x91WV[5\x90c\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\r\x91WV[` `@\x81\x83\x01\x92\x82\x81R\x84Q\x80\x94R\x01\x92\x01\x90_[\x81\x81\x10a\x1F\x90WPPP\x90V[\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x1F\x83V[`@\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\r\xA4W`@RV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\r\xA4W`@RV[`\x01`\x01`@\x1B\x03\x81\x11a\r\xA4W`\x1F\x01`\x1F\x19\x16` \x01\x90V[\x92\x91\x92a \x12\x82a\x1F\xEBV[\x91a  `@Q\x93\x84a\x1F\xCAV[\x82\x94\x81\x84R\x81\x83\x01\x11a\r\x91W\x82\x81` \x93\x84_\x96\x017\x01\x01RV[\x90\x80`\x1F\x83\x01\x12\x15a\r\x91W\x81` a W\x935\x91\x01a \x06V[\x90V[` `\x03\x19\x82\x01\x12a\r\x91W`\x045\x90`\x01`\x01`@\x1B\x03\x82\x11a\r\x91W\x80`#\x83\x01\x12\x15a\r\x91W\x81`$a W\x93`\x04\x015\x91\x01a \x06V[\x80Q\x80\x83R` \x92\x91\x81\x90\x84\x01\x84\x84\x01^_\x82\x82\x01\x84\x01R`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[` \x81R`@c\xFF\xFF\xFF\xFF` a \xDA\x85Q\x84\x83\x87\x01R``\x86\x01\x90a \x95V[\x94\x01Q\x16\x91\x01R\x90V[\x91\x81`\x1F\x84\x01\x12\x15a\r\x91W\x825\x91`\x01`\x01`@\x1B\x03\x83\x11a\r\x91W` \x80\x85\x01\x94\x84`\x05\x1B\x01\x01\x11a\r\x91WV[`\x01`\x01`@\x1B\x03\x81\x11a\r\xA4W`\x05\x1B` \x01\x90V[\x90`\x01\x82\x81\x1C\x92\x16\x80\x15a!YW[` \x83\x10\x14a!EWV[cNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[\x91`\x7F\x16\x91a!:V[\x905`\x1E\x19\x826\x03\x01\x81\x12\x15a\r\x91W\x01` \x815\x91\x01\x91`\x01`\x01`@\x1B\x03\x82\x11a\r\x91W\x816\x03\x83\x13a\r\x91WV[\x90\x80` \x93\x92\x81\x84R\x84\x84\x017_\x82\x82\x01\x84\x01R`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[\x90` c\xFF\xFF\xFF\xFFa!\xE2\x82a!\xDBa!\xCD\x87\x80a!cV[`@\x88R`@\x88\x01\x91a!\x94V[\x95\x01a\x1F\\V[\x16\x91\x01R\x90V[\x15a!\xF0WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`=`$\x82\x01R\x7Fsupplied task does not match the`D\x82\x01R\x7F one recorded in the contract\0\0\0`d\x82\x01R`\x84\x90\xFD[5c\xFF\xFF\xFF\xFF\x81\x16\x81\x03a\r\x91W\x90V[\x90c\xFF\xFF\xFF\xFF\x80\x91\x16\x91\x16\x01\x90c\xFF\xFF\xFF\xFF\x82\x11a\x11\xEDWV[\x80Q\x82\x10\x15a\"\x9AW` \x91`\x05\x1B\x01\x01\x90V[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[\x15a\"\xB5WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FOperator has already responded t`D\x82\x01Rio the task`\xB0\x1B`d\x82\x01R`\x84\x90\xFD[\x81\x81\x10a#\x18WPPV[_\x81U`\x01\x01a#\rV[`@Q\x90a#0\x82a\x1F\xAFV[_` \x83``\x81R\x01RV[`3T`\x01`\x01`\xA0\x1B\x03\x163\x03a#PWV[`d`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R` `$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R\xFD[` \x81\x83\x03\x12a\r\x91W\x80Q\x90`\x01`\x01`@\x1B\x03\x82\x11a\r\x91W\x01\x90` \x82\x82\x03\x12a\r\x91W`@Q\x91` \x83\x01\x83\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\r\xA4W`@R\x80Q\x90`\x01`\x01`@\x1B\x03\x82\x11a\r\x91W\x01\x81`\x1F\x82\x01\x12\x15a\r\x91W\x80Q\x90a$\x01\x82a!\x14V[\x92a$\x0F`@Q\x94\x85a\x1F\xCAV[\x82\x84R` \x80\x85\x01\x93`\x06\x1B\x83\x01\x01\x91\x81\x83\x11a\r\x91W` \x01\x92[\x82\x84\x10a$;WPPPP\x81R\x90V[`@\x84\x83\x03\x12a\r\x91W`@Q\x90a$R\x82a\x1F\xAFV[\x84Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\r\x91W\x82R` \x85\x01Q\x90k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\r\x91W\x82` \x92\x83`@\x95\x01R\x81R\x01\x93\x01\x92a$+V[_\x19\x81\x14a\x11\xEDW`\x01\x01\x90V[\x90a$\xAE\x82a!\x14V[a$\xBB`@Q\x91\x82a\x1F\xCAV[\x82\x81R\x80\x92a$\xCC`\x1F\x19\x91a!\x14V[\x01\x90` 6\x91\x017V[`eT`@\x80Q`\x01`\x01`\xA0\x1B\x03\x80\x84\x16\x82R\x84\x16` \x82\x01R\x91\x92\x91\x7F\xE1\x1C\xDD\xF1\x81jC1\x8C\xA1u\xBB\xC5,\xD0\x18T6\xE9\xCB\xEA\xD7\xC8:\xCCT\xA7>F\x17\x17\xE3\x91\x90\xA1`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x19\x91\x90\x91\x16\x17`eUV[`3\x80T`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x01`\x01`\xA0\x1B\x03\x19\x82\x16\x81\x17\x90\x92U\x90\x91\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0_\x80\xA3V[`eT`\x01`\x01`\xA0\x1B\x03\x163\x03a%\x90WV[c\x8Ey\xFD\xB5`\xE0\x1B_R`\x04_\xFD[\x91\x90\x81\x10\x15a\"\x9AW`\x05\x1B\x81\x015\x90`\xBE\x19\x816\x03\x01\x82\x12\x15a\r\x91W\x01\x90V[\x905\x90`\x1E\x19\x816\x03\x01\x82\x12\x15a\r\x91W\x01\x805\x90`\x01`\x01`@\x1B\x03\x82\x11a\r\x91W` \x01\x91\x81`\x06\x1B6\x03\x83\x13a\r\x91WV[\x91\x90\x82\x01\x80\x92\x11a\x11\xEDWV[5`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\r\x91W\x90V[\x905`\x1E\x19\x826\x03\x01\x81\x12\x15a\r\x91W\x01` \x815\x91\x01\x91`\x01`\x01`@\x1B\x03\x82\x11a\r\x91W\x81`\x06\x1B6\x03\x83\x13a\r\x91WV[\x91` \x90\x82\x81R\x01\x91\x90_\x90[\x80\x82\x10a&eWPPP\x90V[\x90\x91\x92\x835`\x01\x80`\xA0\x1B\x03\x81\x16\x80\x91\x03a\r\x91W\x81R` \x84\x015\x90k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x80\x92\x03a\r\x91W`@\x81`\x01\x93` \x83\x94\x01R\x01\x94\x01\x92\x01\x90a&XV[\x91\x90\x81\x10\x15a\"\x9AW`\x05\x1B\x81\x015\x90`\x9E\x19\x816\x03\x01\x82\x12\x15a\r\x91W\x01\x90V[`@Qc#\xB8r\xDD`\xE0\x1B` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`$\x82\x01R\x92\x90\x91\x16`D\x83\x01R`d\x80\x83\x01\x93\x90\x93R\x91\x81Ra'\x1A\x91a'\x15`\x84\x83a\x1F\xCAV[a'\xDAV[V[`@Qcn\xB1v\x9F`\xE1\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16`$\x82\x01R` \x81\x80`D\x81\x01\x03\x81`\x01`\x01`\xA0\x1B\x03\x86\x16Z\xFA\x90\x81\x15a\r\x86W_\x91a'\xA6W[Pa'\x1A\x93a'p\x91a%\xF6V[`@Qc\t^\xA7\xB3`\xE0\x1B` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x93\x16`$\x84\x01R`D\x80\x84\x01\x91\x90\x91R\x82Ra'\x15`d\x83a\x1F\xCAV[\x90P` \x81=` \x11a'\xD2W[\x81a'\xC1` \x93\x83a\x1F\xCAV[\x81\x01\x03\x12a\r\x91WQa'\x1Aa'bV[=\x91Pa'\xB4V[\x90a(Y\x91`\x01\x80`\xA0\x1B\x03\x16_\x80`@Q\x93a'\xF8`@\x86a\x1F\xCAV[` \x85R\x7FSafeERC20: low-level call failed` \x86\x01R` \x81Q\x91\x01\x82\x85Z\xF1=\x15a(\xEAW=\x91a(=\x83a\x1F\xEBV[\x92a(K`@Q\x94\x85a\x1F\xCAV[\x83R=_` \x85\x01>a(\xEEV[\x80Q\x90\x81\x15\x91\x82\x15a(\xC7W[PP\x15a(oWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FSafeERC20: ERC20 operation did n`D\x82\x01Ri\x1B\xDD\x08\x1C\xDDX\xD8\xD9YY`\xB2\x1B`d\x82\x01R`\x84\x90\xFD[\x81\x92P\x90` \x91\x81\x01\x03\x12a\r\x91W` \x01Q\x80\x15\x15\x81\x03a\r\x91W_\x80a(fV[``\x91[\x91\x92\x90\x15a)PWP\x81Q\x15a)\x02WP\x90V[;\x15a)\x0BW\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R`d\x90\xFD[\x82Q\x90\x91P\x15a)cWP\x80Q\x90` \x01\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R\x90\x81\x90a)\x87\x90`$\x83\x01\x90a \x95V[\x03\x90\xFD\xFE\xA2dipfsX\"\x12 ;TcB\xEF!\xA7%]\r{t\x11\x81\x8Ce\xD3\x8E\xDB\xCD\xFC\x1Ex\x86)\x16\x9D\x92\xE5\xBA\x18\xBCdsolcC\0\x08\x1B\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x60806040526004361015610011575f80fd5b5f5f3560e01c80631785f53c14611e865780631fdb0cfd14611ea8578063279432eb14611e865780632d89f6fc14611e5157806333cfb7b714611b835780633415a49c146115245780633bc28c8c14611500578063485cc9551461135e578063683048351461131a5780636b3aa72e146112d6578063715018a61461127b578063733d056c1461123b5780637b6c42361461120157806385edf874146111225780638b00ce7c146110ff5780638da5cb5b146110d75780639677de1014610db85780639926ee7d14610c245780639da16d8e14610c01578063a0169ddd14610b74578063a20b99bf146108d2578063a364f4da1461080d578063a98fb35514610786578063ba5508801461076c578063c1a8e2c5146106d7578063c20bab7f146105d8578063ca8aa7c714610593578063e481af9d146104bd578063f25f161014610424578063f2fde38b14610393578063fc299dee1461036a5763fce36c7d1461017a575f80fd5b34610257576020366003190112610257576004356001600160401b038111610366576101aa9036906004016120e4565b6101b592919261257c565b7f0000000000000000000000000000000000000000000000000000000000000000825b82811061030257506001600160a01b031690813b156102fe576040519363fce36c7d60e01b8552816024860160206004880152526044850160448360051b87010192828690609e19813603015b8383106102655788808b8181808c0381838f5af1801561025a576102465750f35b8161025091611fca565b6102575780f35b80fd5b6040513d84823e3d90fd5b9091929394956043198a82030186528635828112156102fa5760206001928582930190608063ffffffff6102e8826102ae6102a08780612617565b60a0885260a088019161264b565b95898060a01b036102c0898301611ed7565b168887015260408101356040870152836102dc60608301611f5c565b16606087015201611f5c565b16910152980196019493019190610225565b8980fd5b8280fd5b8061033761031e6020610318600195888b6126af565b01612603565b604061032b84888b6126af565b013590309033906126d1565b61036061034a602061031884888b6126af565b84604061035885898c6126af565b01359161271c565b016101d8565b5080fd5b50346102575780600319360112610257576065546040516001600160a01b039091168152602090f35b5034610257576020366003190112610257576103ad611ec1565b6103b561233c565b6001600160a01b038116156103d0576103cd90612534565b80f35b60405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b6064820152608490fd5b503461025757602036600319011261025757806004356001600160a01b038116908190036104ba5761045461233c565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690813b156104b85782916044839260405194859384926334f65bfd60e21b845230600485015260248401525af1801561025a576102465750f35b505b50fd5b50346102575780600319360112610257576040516302e0740360e31b815281816004817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa90811561025a578291610571575b506105268151516124a4565b915b8151805182101561055f57600191906001600160a01b039061054b908390612286565b5151166105588286612286565b5201610528565b6040518061056d8682611f6d565b0390f35b61058d91503d8084833e6105858183611fca565b810190612394565b5f61051a565b50346102575780600319360112610257576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b5034610257576040366003190112610257576105f2611ec1565b6105fa611f49565b9060018060a01b03168252609960205263ffffffff6040832091165f5260205260405f209060405191818154916106308361212b565b80865292600181169081156106ad575060011461066c575b61056d8561065881870382611fca565b604051918291602083526020830190612095565b815260208120939250905b808210610693575090915081016020016106588261056d610648565b919260018160209254838588010152019101909291610677565b86955061056d9693506020925061065894915060ff191682840152151560051b8201019293610648565b5034610257576040366003190112610257576106f1611ec1565b506024356001600160401b03811161036657366023820112156103665780600401356024602061072083612114565b61072d6040519182611fca565b838152019160051b8301019136831161076857602401905b828210610750578380f35b6020809161075d84611f5c565b815201910190610745565b8380fd5b50346102575761077b36611eeb565b5050506103cd61233c565b503461025757806107963661205a565b61079e61233c565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316803b156104b85760405163a98fb35560e01b81526020600482015291839183918290849082906107fc906024830190612095565b03925af1801561025a576102465750f35b503461025757602036600319011261025757610827611ec1565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031633036108c35781907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690813b156104b8576040516351b27a6d60e11b81526001600160a01b0390911660048201529082908290602490829084905af1801561025a576102465750f35b6346bf228160e01b8252600482fd5b5034610257576020366003190112610257576004356001600160401b038111610366576109039036906004016120e4565b61090e92919261257c565b7f0000000000000000000000000000000000000000000000000000000000000000825b828110610abf5750919283926001600160a01b031690813b15610aba57935060405191634e5cd2fd60e11b835280604484013060048601526040602486015252606483019060648160051b850101958092869160be19813603015b8484106109ad578880898181808f0381838e5af1801561025a576102465750f35b878a036063190183528535818112156102fa578201996109de6109d08c80612617565b60c0845260c084019161264b565b9a6001600160a01b036109f360208301611ed7565b1660208301526020610a086040830183612617565b848f036040860152808f529d9091019c8c5b818110610a89575050506020610a798c9d600194610a6b8563ffffffff610a446060889901611f5c565b16606084015263ffffffff610a5b60808301611f5c565b16608084015260a0810190612163565b9160a0818503910152612194565b980194019401939295995061098c565b90919d60408f60019260208392858060a01b03610aa582611ed7565b16835201356020820152019f01929101610a1a565b505050fd5b909291829483955b610adf610ad585858561259f565b60408101906125c1565b9050871015610b2f57610af6610ad585858561259f565b881015610b1b576001916020610b13928a60061b010135906125f6565b960195610ac7565b634e487b7160e01b86526032600452602486fd5b600192949593919650610b6e90610b5881308a610b536020610318898d339561259f565b6126d1565b84610b696020610318868a8d61259f565b61271c565b01610931565b50346102575760203660031901126102575780610b8f611ec1565b610b9761233c565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690813b156104b85760405163a0169ddd60e01b81526001600160a01b0390911660048201529082908290602490829084905af1801561025a576102465750f35b503461025757602036600319011261025757610c1b611ec1565b506103cd61233c565b5034610d91576040366003190112610d9157610c3e611ec1565b602435906001600160401b038211610d915760606003198336030112610d915760405190606082018281106001600160401b03821117610da45760405282600401356001600160401b038111610d9157610c9e906004369186010161203c565b8252602082019160248401358352604460408201940135845260018060a01b037f0000000000000000000000000000000000000000000000000000000000000000163303610d95577f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690813b15610d91575f610d5993819560405197889687958694639926ee7d60e01b865260018060a01b0316600486015260406024860152516060604486015260a4850190612095565b9151606484015251608483015203925af18015610d8657610d78575080f35b610d8491505f90611fca565b005b6040513d5f823e3d90fd5b5f80fd5b6346bf228160e01b5f5260045ffd5b634e487b7160e01b5f52604160045260245ffd5b34610d91576060366003190112610d91576004356001600160401b038111610d915760406003198236030112610d9157610df0611f49565b906044356001600160a01b0381169190829003610d9157610e5663ffffffff604051602081019060208252610e3d81610e2f60408201886004016121b4565b03601f198101835282611fca565b519020941693845f52609860205260405f2054146121e9565b825f52609a60205260ff60405f20541661108757602490825f52609960205260405f20845f52602052610e95610e8f60405f205461212b565b156122ae565b0163ffffffff610ece610ea78361225b565b7f00000000000000000000000000000000000000000000000000000000000000009061226c565b1643111561103357610edf9061225b565b604051630955f2d960e41b81526004810183905263ffffffff9190911660248201526020816044817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa908115610d86575f91611001575b5015610fa2575f52609960205260405f20905f5260205260405f20610f66815461212b565b601f8111610f82575b50600e661cdb185cda195960ca1b019055005b610f9c90825f52601f60205f20910160051c81019061230d565b81610f6f565b60405162461bcd60e51b815260206004820152603160248201527f4f70657261746f7220776173206e6f742072656769737465726564207768656e604482015270081d185cdac81dd85cc818dc99585d1959607a1b6064820152608490fd5b90506020813d60201161102b575b8161101c60209383611fca565b81010312610d91575183610f41565b3d915061100f565b60405162461bcd60e51b815260206004820152602660248201527f5461736b20726573706f6e73652074696d6520686173206e6f742065787069726044820152651959081e595d60d21b6064820152608490fd5b60405162461bcd60e51b815260206004820152602260248201527f5461736b2068617320616c7265616479206265656e20726573706f6e64656420604482015261746f60f01b6064820152608490fd5b34610d91575f366003190112610d91576033546040516001600160a01b039091168152602090f35b34610d91575f366003190112610d9157602063ffffffff60975416604051908152f35b34610d91576111303661205a565b611138612323565b50611141612323565b90815263ffffffff43166020820152604051602081019061116681610e2f85856120b9565b51902063ffffffff609754165f52609860205260405f205560975490600163ffffffff8316807f58180a6a0403a63c2b5ce4b85d129d46a80d37851b2216bd0a98b59e7309b847604051806111bb87826120b9565b0390a2019163ffffffff83116111ed5763ffffffff61056d93169063ffffffff191617609755604051918291826120b9565b634e487b7160e01b5f52601160045260245ffd5b34610d91576020366003190112610d915763ffffffff61121f611f36565b165f52609a602052602060ff60405f2054166040519015158152f35b34610d91575f366003190112610d9157602060405163ffffffff7f0000000000000000000000000000000000000000000000000000000000000000168152f35b34610d91575f366003190112610d915761129361233c565b603380546001600160a01b031981169091555f906001600160a01b03167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e08280a3005b34610d91575f366003190112610d91576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b34610d91575f366003190112610d91576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b34610d91576040366003190112610d9157611377611ec1565b602435906001600160a01b0382168203610d91575f549160ff8360081c1615928380946114f3575b80156114dc575b156114805760ff1981166001175f558361146f575b5060ff5f5460081c1615611416576113d56113da92612534565b6124d6565b6113e057005b61ff00195f54165f557f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb3847402498602060405160018152a1005b60405162461bcd60e51b815260206004820152602b60248201527f496e697469616c697a61626c653a20636f6e7472616374206973206e6f74206960448201526a6e697469616c697a696e6760a81b6064820152608490fd5b61ffff1916610101175f55836113bb565b60405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b6064820152608490fd5b50303b1580156113a65750600160ff8216146113a6565b50600160ff82161061139f565b34610d91576020366003190112610d9157610d8461151c611ec1565b6113d561233c565b34610d91576060366003190112610d91576004356001600160401b038111610d915736819003600482016040600319830112610d9157611562611f49565b906044356001600160401b038111610d915761158290369060040161203c565b926115a863ffffffff604051602081019060208252610e3d81610e2f60408201896121b4565b602485019463ffffffff6115be610ea78861225b565b164311611b2f5782359160221901821215610d9157016004810135906001600160401b038211610d9157602401908036038213610d915761162c602760405183819460208301966602432b6363796160cd1b88528484013781015f838201520301601f198101835282611fca565b5190207f19457468657265756d205369676e6564204d6573736167653a0a3332000000005f52601c52603c5f2092805181019060608160208401930312610d915760208101516001600160401b038111610d915781019582603f88011215610d9157602087015161169c81612114565b976116aa604051998a611fca565b818952602080808b019360051b8301010190858211610d9157604001915b818310611b0f5750505060408201516001600160401b038111610d915782019280603f85011215610d915760208401519361170285612114565b946117106040519687611fca565b8086526020808088019260051b8401010191838311610d915760408101915b838310611aa557505050505060608201519063ffffffff8216809203610d915761175d63ffffffff9161225b565b1603611a49575f5b8651811015611932576001600160a01b036117808289612286565b51165f52609960205260405f20855f526020526117a3610e8f60405f205461212b565b6117ad8184612286565b516001600160a01b036117c0838a612286565b51165f52609960205260405f20865f5260205260405f20908051906001600160401b038211610da4576117f3835461212b565b601f81116118f7575b50602090601f8311600114611890576001949392915f9183611885575b50505f19600383901b1c191690841b1790555b857f8eb2d2fcccf5801e10ff58cd73e8781ba923122963789378771f03c1148b023e838060a01b0361185e848c612286565b5116604051809160408252611876604083018b6121b4565b9060208301520390a201611765565b015190508b80611819565b90601f19831691845f52815f20925f5b8181106118df5750916001969594929183889593106118c7575b505050811b01905561182c565b01515f1960f88460031b161c191690558b80806118ba565b929360206001819287860151815501950193016118a0565b61192290845f5260205f20601f850160051c81019160208610611928575b601f0160051c019061230d565b8a6117fc565b9091508190611915565b61197760208388885f52609a835260405f20600160ff198254161790556040519384928392630b135d3f60e11b84526004840152604060248401526044830190612095565b03817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa908115610d86575f91611a06575b506001600160e01b031916630b135d3f60e11b036119cd57005b60405162461bcd60e51b8152602060048201526011602482015270496e76616c6964207369676e617475726560781b6044820152606490fd5b90506020813d602011611a41575b81611a2160209383611fca565b81010312610d9157516001600160e01b031981168103610d9157816119b3565b3d9150611a14565b60405162461bcd60e51b815260206004820152602e60248201527f5265666572656e636520626c6f636b206d757374206d61746368207461736b2060448201526d6372656174696f6e20626c6f636b60901b6064820152608490fd5b82516001600160401b038111610d915760209083010185603f82011215610d9157602081015191611ad583611feb565b611ae26040519182611fca565b8381526040838501018810610d91575f602085819660408397018386015e8301015281520192019161172f565b82516001600160a01b0381168103610d91578152602092830192016116c8565b60405162461bcd60e51b815260206004820152602660248201527f5461736b20726573706f6e73652074696d652068617320616c726561647920656044820152651e1c1a5c995960d21b6064820152608490fd5b34610d91576020366003190112610d9157611b9c611ec1565b6040516302e0740360e31b81525f816004817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa908115610d86575f91611e37575b509081515190611bf782612114565b90611c056040519283611fca565b828252611c1183612114565b602083019490601f19013686375f5b848110611e0957505060408051639004134760e01b81526001600160a01b0390921660048301526024820152815160448201819052909384916064830191905f5b818110611de757505f939283900391508290507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa928315610d86575f93611d55575b505f5f5b838110611d295750611cc3906124a4565b925f905f5b848110611cdd576040518061056d8882611f6d565b611ce78183612286565b51611cf5575b600101611cc8565b91600190611d21906001600160a01b03611d0f8688612286565b5116611d1b828a612286565b52612496565b929050611ced565b611d338186612286565b51611d41575b600101611cb2565b90611d4d600191612496565b919050611d39565b9092503d805f833e611d678183611fca565b810190602081830312610d91578051906001600160401b038211610d9157019080601f83011215610d91578151611d9d81612114565b92611dab6040519485611fca565b81845260208085019260051b820101928311610d9157602001905b828210611dd7575050509183611cae565b8151815260209182019101611dc6565b82516001600160a01b0316845287945060209384019390920191600101611c61565b600190818060a09895981b03611e20828551612286565b515116611e2d8287612286565b5201949194611c20565b611e4b91503d805f833e6105858183611fca565b82611be8565b34610d91576020366003190112610d915763ffffffff611e6f611f36565b165f526098602052602060405f2054604051908152f35b34610d91576020366003190112610d9157611e9f611ec1565b50610d8461233c565b34610d9157611eb636611eeb565b505050610d8461233c565b600435906001600160a01b0382168203610d9157565b35906001600160a01b0382168203610d9157565b6060906003190112610d91576004356001600160a01b0381168103610d9157906024356001600160a01b0381168103610d9157906044356001600160e01b031981168103610d915790565b6004359063ffffffff82168203610d9157565b6024359063ffffffff82168203610d9157565b359063ffffffff82168203610d9157565b60206040818301928281528451809452019201905f5b818110611f905750505090565b82516001600160a01b0316845260209384019390920191600101611f83565b604081019081106001600160401b03821117610da457604052565b90601f801991011681019081106001600160401b03821117610da457604052565b6001600160401b038111610da457601f01601f191660200190565b92919261201282611feb565b916120206040519384611fca565b829481845281830111610d91578281602093845f960137010152565b9080601f83011215610d915781602061205793359101612006565b90565b6020600319820112610d9157600435906001600160401b038211610d915780602383011215610d915781602461205793600401359101612006565b805180835260209291819084018484015e5f828201840152601f01601f1916010190565b60208152604063ffffffff60206120da855184838701526060860190612095565b9401511691015290565b9181601f84011215610d91578235916001600160401b038311610d91576020808501948460051b010111610d9157565b6001600160401b038111610da45760051b60200190565b90600182811c92168015612159575b602083101461214557565b634e487b7160e01b5f52602260045260245ffd5b91607f169161213a565b9035601e1982360301811215610d915701602081359101916001600160401b038211610d91578136038313610d9157565b908060209392818452848401375f828201840152601f01601f1916010190565b90602063ffffffff6121e2826121db6121cd8780612163565b604088526040880191612194565b9501611f5c565b1691015290565b156121f057565b60405162461bcd60e51b815260206004820152603d60248201527f737570706c696564207461736b20646f6573206e6f74206d617463682074686560448201527f206f6e65207265636f7264656420696e2074686520636f6e74726163740000006064820152608490fd5b3563ffffffff81168103610d915790565b9063ffffffff8091169116019063ffffffff82116111ed57565b805182101561229a5760209160051b010190565b634e487b7160e01b5f52603260045260245ffd5b156122b557565b60405162461bcd60e51b815260206004820152602a60248201527f4f70657261746f722068617320616c726561647920726573706f6e64656420746044820152696f20746865207461736b60b01b6064820152608490fd5b818110612318575050565b5f815560010161230d565b6040519061233082611faf565b5f602083606081520152565b6033546001600160a01b0316330361235057565b606460405162461bcd60e51b815260206004820152602060248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152fd5b602081830312610d91578051906001600160401b038211610d91570190602082820312610d915760405191602083018381106001600160401b03821117610da4576040528051906001600160401b038211610d91570181601f82011215610d915780519061240182612114565b9261240f6040519485611fca565b82845260208085019360061b83010191818311610d9157602001925b82841061243b5750505050815290565b604084830312610d91576040519061245282611faf565b84516001600160a01b0381168103610d915782526020850151906bffffffffffffffffffffffff82168203610d91578260209283604095015281520193019261242b565b5f1981146111ed5760010190565b906124ae82612114565b6124bb6040519182611fca565b82815280926124cc601f1991612114565b0190602036910137565b606554604080516001600160a01b038084168252841660208201529192917fe11cddf1816a43318ca175bbc52cd0185436e9cbead7c83acc54a73e461717e39190a16001600160a01b03166001600160a01b03199190911617606555565b603380546001600160a01b039283166001600160a01b0319821681179092559091167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e05f80a3565b6065546001600160a01b0316330361259057565b638e79fdb560e01b5f5260045ffd5b919081101561229a5760051b8101359060be1981360301821215610d91570190565b903590601e1981360301821215610d9157018035906001600160401b038211610d9157602001918160061b36038313610d9157565b919082018092116111ed57565b356001600160a01b0381168103610d915790565b9035601e1982360301811215610d915701602081359101916001600160401b038211610d91578160061b36038313610d9157565b916020908281520191905f905b8082106126655750505090565b909192833560018060a01b038116809103610d915781526020840135906bffffffffffffffffffffffff8216809203610d9157604081600193602083940152019401920190612658565b919081101561229a5760051b81013590609e1981360301821215610d91570190565b6040516323b872dd60e01b60208201526001600160a01b03928316602482015292909116604483015260648083019390935291815261271a91612715608483611fca565b6127da565b565b604051636eb1769f60e11b81523060048201526001600160a01b0383166024820152602081806044810103816001600160a01b0386165afa908115610d86575f916127a6575b5061271a93612770916125f6565b60405163095ea7b360e01b60208201526001600160a01b0390931660248401526044808401919091528252612715606483611fca565b90506020813d6020116127d2575b816127c160209383611fca565b81010312610d91575161271a612762565b3d91506127b4565b906128599160018060a01b03165f80604051936127f8604086611fca565b602085527f5361666545524332303a206c6f772d6c6576656c2063616c6c206661696c6564602086015260208151910182855af13d156128ea573d9161283d83611feb565b9261284b6040519485611fca565b83523d5f602085013e6128ee565b80519081159182156128c7575b50501561286f57565b60405162461bcd60e51b815260206004820152602a60248201527f5361666545524332303a204552433230206f7065726174696f6e20646964206e6044820152691bdd081cdd58d8d9595960b21b6064820152608490fd5b8192509060209181010312610d9157602001518015158103610d91575f80612866565b6060915b919290156129505750815115612902575090565b3b1561290b5790565b60405162461bcd60e51b815260206004820152601d60248201527f416464726573733a2063616c6c20746f206e6f6e2d636f6e74726163740000006044820152606490fd5b8251909150156129635750805190602001fd5b60405162461bcd60e51b815260206004820152908190612987906024830190612095565b0390fdfea26469706673582212203b546342ef21a7255d0d7b7411818c65d38edbcdfc1e788629169d92e5ba18bc64736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10\x15a\0\x11W_\x80\xFD[__5`\xE0\x1C\x80c\x17\x85\xF5<\x14a\x1E\x86W\x80c\x1F\xDB\x0C\xFD\x14a\x1E\xA8W\x80c'\x942\xEB\x14a\x1E\x86W\x80c-\x89\xF6\xFC\x14a\x1EQW\x80c3\xCF\xB7\xB7\x14a\x1B\x83W\x80c4\x15\xA4\x9C\x14a\x15$W\x80c;\xC2\x8C\x8C\x14a\x15\0W\x80cH\\\xC9U\x14a\x13^W\x80ch0H5\x14a\x13\x1AW\x80ck:\xA7.\x14a\x12\xD6W\x80cqP\x18\xA6\x14a\x12{W\x80cs=\x05l\x14a\x12;W\x80c{lB6\x14a\x12\x01W\x80c\x85\xED\xF8t\x14a\x11\"W\x80c\x8B\0\xCE|\x14a\x10\xFFW\x80c\x8D\xA5\xCB[\x14a\x10\xD7W\x80c\x96w\xDE\x10\x14a\r\xB8W\x80c\x99&\xEE}\x14a\x0C$W\x80c\x9D\xA1m\x8E\x14a\x0C\x01W\x80c\xA0\x16\x9D\xDD\x14a\x0BtW\x80c\xA2\x0B\x99\xBF\x14a\x08\xD2W\x80c\xA3d\xF4\xDA\x14a\x08\rW\x80c\xA9\x8F\xB3U\x14a\x07\x86W\x80c\xBAU\x08\x80\x14a\x07lW\x80c\xC1\xA8\xE2\xC5\x14a\x06\xD7W\x80c\xC2\x0B\xAB\x7F\x14a\x05\xD8W\x80c\xCA\x8A\xA7\xC7\x14a\x05\x93W\x80c\xE4\x81\xAF\x9D\x14a\x04\xBDW\x80c\xF2_\x16\x10\x14a\x04$W\x80c\xF2\xFD\xE3\x8B\x14a\x03\x93W\x80c\xFC)\x9D\xEE\x14a\x03jWc\xFC\xE3l}\x14a\x01zW_\x80\xFD[4a\x02WW` 6`\x03\x19\x01\x12a\x02WW`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x03fWa\x01\xAA\x906\x90`\x04\x01a \xE4V[a\x01\xB5\x92\x91\x92a%|V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82[\x82\x81\x10a\x03\x02WP`\x01`\x01`\xA0\x1B\x03\x16\x90\x81;\x15a\x02\xFEW`@Q\x93c\xFC\xE3l}`\xE0\x1B\x85R\x81`$\x86\x01` `\x04\x88\x01RR`D\x85\x01`D\x83`\x05\x1B\x87\x01\x01\x92\x82\x86\x90`\x9E\x19\x816\x03\x01[\x83\x83\x10a\x02eW\x88\x80\x8B\x81\x81\x80\x8C\x03\x81\x83\x8FZ\xF1\x80\x15a\x02ZWa\x02FWP\xF3[\x81a\x02P\x91a\x1F\xCAV[a\x02WW\x80\xF3[\x80\xFD[`@Q=\x84\x82>=\x90\xFD[\x90\x91\x92\x93\x94\x95`C\x19\x8A\x82\x03\x01\x86R\x865\x82\x81\x12\x15a\x02\xFAW` `\x01\x92\x85\x82\x93\x01\x90`\x80c\xFF\xFF\xFF\xFFa\x02\xE8\x82a\x02\xAEa\x02\xA0\x87\x80a&\x17V[`\xA0\x88R`\xA0\x88\x01\x91a&KV[\x95\x89\x80`\xA0\x1B\x03a\x02\xC0\x89\x83\x01a\x1E\xD7V[\x16\x88\x87\x01R`@\x81\x015`@\x87\x01R\x83a\x02\xDC``\x83\x01a\x1F\\V[\x16``\x87\x01R\x01a\x1F\\V[\x16\x91\x01R\x98\x01\x96\x01\x94\x93\x01\x91\x90a\x02%V[\x89\x80\xFD[\x82\x80\xFD[\x80a\x037a\x03\x1E` a\x03\x18`\x01\x95\x88\x8Ba&\xAFV[\x01a&\x03V[`@a\x03+\x84\x88\x8Ba&\xAFV[\x015\x900\x903\x90a&\xD1V[a\x03`a\x03J` a\x03\x18\x84\x88\x8Ba&\xAFV[\x84`@a\x03X\x85\x89\x8Ca&\xAFV[\x015\x91a'\x1CV[\x01a\x01\xD8V[P\x80\xFD[P4a\x02WW\x80`\x03\x196\x01\x12a\x02WW`eT`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[P4a\x02WW` 6`\x03\x19\x01\x12a\x02WWa\x03\xADa\x1E\xC1V[a\x03\xB5a#<V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a\x03\xD0Wa\x03\xCD\x90a%4V[\x80\xF3[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x90\xFD[P4a\x02WW` 6`\x03\x19\x01\x12a\x02WW\x80`\x045`\x01`\x01`\xA0\x1B\x03\x81\x16\x90\x81\x90\x03a\x04\xBAWa\x04Ta#<V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90\x81;\x15a\x04\xB8W\x82\x91`D\x83\x92`@Q\x94\x85\x93\x84\x92c4\xF6[\xFD`\xE2\x1B\x84R0`\x04\x85\x01R`$\x84\x01RZ\xF1\x80\x15a\x02ZWa\x02FWP\xF3[P[P\xFD[P4a\x02WW\x80`\x03\x196\x01\x12a\x02WW`@Qc\x02\xE0t\x03`\xE3\x1B\x81R\x81\x81`\x04\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x02ZW\x82\x91a\x05qW[Pa\x05&\x81QQa$\xA4V[\x91[\x81Q\x80Q\x82\x10\x15a\x05_W`\x01\x91\x90`\x01`\x01`\xA0\x1B\x03\x90a\x05K\x90\x83\x90a\"\x86V[QQ\x16a\x05X\x82\x86a\"\x86V[R\x01a\x05(V[`@Q\x80a\x05m\x86\x82a\x1FmV[\x03\x90\xF3[a\x05\x8D\x91P=\x80\x84\x83>a\x05\x85\x81\x83a\x1F\xCAV[\x81\x01\x90a#\x94V[_a\x05\x1AV[P4a\x02WW\x80`\x03\x196\x01\x12a\x02WW`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[P4a\x02WW`@6`\x03\x19\x01\x12a\x02WWa\x05\xF2a\x1E\xC1V[a\x05\xFAa\x1FIV[\x90`\x01\x80`\xA0\x1B\x03\x16\x82R`\x99` Rc\xFF\xFF\xFF\xFF`@\x83 \x91\x16_R` R`@_ \x90`@Q\x91\x81\x81T\x91a\x060\x83a!+V[\x80\x86R\x92`\x01\x81\x16\x90\x81\x15a\x06\xADWP`\x01\x14a\x06lW[a\x05m\x85a\x06X\x81\x87\x03\x82a\x1F\xCAV[`@Q\x91\x82\x91` \x83R` \x83\x01\x90a \x95V[\x81R` \x81 \x93\x92P\x90[\x80\x82\x10a\x06\x93WP\x90\x91P\x81\x01` \x01a\x06X\x82a\x05ma\x06HV[\x91\x92`\x01\x81` \x92T\x83\x85\x88\x01\x01R\x01\x91\x01\x90\x92\x91a\x06wV[\x86\x95Pa\x05m\x96\x93P` \x92Pa\x06X\x94\x91P`\xFF\x19\x16\x82\x84\x01R\x15\x15`\x05\x1B\x82\x01\x01\x92\x93a\x06HV[P4a\x02WW`@6`\x03\x19\x01\x12a\x02WWa\x06\xF1a\x1E\xC1V[P`$5`\x01`\x01`@\x1B\x03\x81\x11a\x03fW6`#\x82\x01\x12\x15a\x03fW\x80`\x04\x015`$` a\x07 \x83a!\x14V[a\x07-`@Q\x91\x82a\x1F\xCAV[\x83\x81R\x01\x91`\x05\x1B\x83\x01\x01\x916\x83\x11a\x07hW`$\x01\x90[\x82\x82\x10a\x07PW\x83\x80\xF3[` \x80\x91a\x07]\x84a\x1F\\V[\x81R\x01\x91\x01\x90a\x07EV[\x83\x80\xFD[P4a\x02WWa\x07{6a\x1E\xEBV[PPPa\x03\xCDa#<V[P4a\x02WW\x80a\x07\x966a ZV[a\x07\x9Ea#<V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x04\xB8W`@Qc\xA9\x8F\xB3U`\xE0\x1B\x81R` `\x04\x82\x01R\x91\x83\x91\x83\x91\x82\x90\x84\x90\x82\x90a\x07\xFC\x90`$\x83\x01\x90a \x95V[\x03\x92Z\xF1\x80\x15a\x02ZWa\x02FWP\xF3[P4a\x02WW` 6`\x03\x19\x01\x12a\x02WWa\x08'a\x1E\xC1V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x163\x03a\x08\xC3W\x81\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90\x81;\x15a\x04\xB8W`@QcQ\xB2zm`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R\x90\x82\x90\x82\x90`$\x90\x82\x90\x84\x90Z\xF1\x80\x15a\x02ZWa\x02FWP\xF3[cF\xBF\"\x81`\xE0\x1B\x82R`\x04\x82\xFD[P4a\x02WW` 6`\x03\x19\x01\x12a\x02WW`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x03fWa\t\x03\x906\x90`\x04\x01a \xE4V[a\t\x0E\x92\x91\x92a%|V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82[\x82\x81\x10a\n\xBFWP\x91\x92\x83\x92`\x01`\x01`\xA0\x1B\x03\x16\x90\x81;\x15a\n\xBAW\x93P`@Q\x91cN\\\xD2\xFD`\xE1\x1B\x83R\x80`D\x84\x010`\x04\x86\x01R`@`$\x86\x01RR`d\x83\x01\x90`d\x81`\x05\x1B\x85\x01\x01\x95\x80\x92\x86\x91`\xBE\x19\x816\x03\x01[\x84\x84\x10a\t\xADW\x88\x80\x89\x81\x81\x80\x8F\x03\x81\x83\x8EZ\xF1\x80\x15a\x02ZWa\x02FWP\xF3[\x87\x8A\x03`c\x19\x01\x83R\x855\x81\x81\x12\x15a\x02\xFAW\x82\x01\x99a\t\xDEa\t\xD0\x8C\x80a&\x17V[`\xC0\x84R`\xC0\x84\x01\x91a&KV[\x9A`\x01`\x01`\xA0\x1B\x03a\t\xF3` \x83\x01a\x1E\xD7V[\x16` \x83\x01R` a\n\x08`@\x83\x01\x83a&\x17V[\x84\x8F\x03`@\x86\x01R\x80\x8FR\x9D\x90\x91\x01\x9C\x8C[\x81\x81\x10a\n\x89WPPP` a\ny\x8C\x9D`\x01\x94a\nk\x85c\xFF\xFF\xFF\xFFa\nD``\x88\x99\x01a\x1F\\V[\x16``\x84\x01Rc\xFF\xFF\xFF\xFFa\n[`\x80\x83\x01a\x1F\\V[\x16`\x80\x84\x01R`\xA0\x81\x01\x90a!cV[\x91`\xA0\x81\x85\x03\x91\x01Ra!\x94V[\x98\x01\x94\x01\x94\x01\x93\x92\x95\x99Pa\t\x8CV[\x90\x91\x9D`@\x8F`\x01\x92` \x83\x92\x85\x80`\xA0\x1B\x03a\n\xA5\x82a\x1E\xD7V[\x16\x83R\x015` \x82\x01R\x01\x9F\x01\x92\x91\x01a\n\x1AV[PPP\xFD[\x90\x92\x91\x82\x94\x83\x95[a\n\xDFa\n\xD5\x85\x85\x85a%\x9FV[`@\x81\x01\x90a%\xC1V[\x90P\x87\x10\x15a\x0B/Wa\n\xF6a\n\xD5\x85\x85\x85a%\x9FV[\x88\x10\x15a\x0B\x1BW`\x01\x91` a\x0B\x13\x92\x8A`\x06\x1B\x01\x015\x90a%\xF6V[\x96\x01\x95a\n\xC7V[cNH{q`\xE0\x1B\x86R`2`\x04R`$\x86\xFD[`\x01\x92\x94\x95\x93\x91\x96Pa\x0Bn\x90a\x0BX\x810\x8Aa\x0BS` a\x03\x18\x89\x8D3\x95a%\x9FV[a&\xD1V[\x84a\x0Bi` a\x03\x18\x86\x8A\x8Da%\x9FV[a'\x1CV[\x01a\t1V[P4a\x02WW` 6`\x03\x19\x01\x12a\x02WW\x80a\x0B\x8Fa\x1E\xC1V[a\x0B\x97a#<V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90\x81;\x15a\x04\xB8W`@Qc\xA0\x16\x9D\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R\x90\x82\x90\x82\x90`$\x90\x82\x90\x84\x90Z\xF1\x80\x15a\x02ZWa\x02FWP\xF3[P4a\x02WW` 6`\x03\x19\x01\x12a\x02WWa\x0C\x1Ba\x1E\xC1V[Pa\x03\xCDa#<V[P4a\r\x91W`@6`\x03\x19\x01\x12a\r\x91Wa\x0C>a\x1E\xC1V[`$5\x90`\x01`\x01`@\x1B\x03\x82\x11a\r\x91W```\x03\x19\x836\x03\x01\x12a\r\x91W`@Q\x90``\x82\x01\x82\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\r\xA4W`@R\x82`\x04\x015`\x01`\x01`@\x1B\x03\x81\x11a\r\x91Wa\x0C\x9E\x90`\x046\x91\x86\x01\x01a <V[\x82R` \x82\x01\x91`$\x84\x015\x83R`D`@\x82\x01\x94\x015\x84R`\x01\x80`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x163\x03a\r\x95W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90\x81;\x15a\r\x91W_a\rY\x93\x81\x95`@Q\x97\x88\x96\x87\x95\x86\x94c\x99&\xEE}`\xE0\x1B\x86R`\x01\x80`\xA0\x1B\x03\x16`\x04\x86\x01R`@`$\x86\x01RQ```D\x86\x01R`\xA4\x85\x01\x90a \x95V[\x91Q`d\x84\x01RQ`\x84\x83\x01R\x03\x92Z\xF1\x80\x15a\r\x86Wa\rxWP\x80\xF3[a\r\x84\x91P_\x90a\x1F\xCAV[\0[`@Q=_\x82>=\x90\xFD[_\x80\xFD[cF\xBF\"\x81`\xE0\x1B_R`\x04_\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[4a\r\x91W``6`\x03\x19\x01\x12a\r\x91W`\x045`\x01`\x01`@\x1B\x03\x81\x11a\r\x91W`@`\x03\x19\x826\x03\x01\x12a\r\x91Wa\r\xF0a\x1FIV[\x90`D5`\x01`\x01`\xA0\x1B\x03\x81\x16\x91\x90\x82\x90\x03a\r\x91Wa\x0EVc\xFF\xFF\xFF\xFF`@Q` \x81\x01\x90` \x82Ra\x0E=\x81a\x0E/`@\x82\x01\x88`\x04\x01a!\xB4V[\x03`\x1F\x19\x81\x01\x83R\x82a\x1F\xCAV[Q\x90 \x94\x16\x93\x84_R`\x98` R`@_ T\x14a!\xE9V[\x82_R`\x9A` R`\xFF`@_ T\x16a\x10\x87W`$\x90\x82_R`\x99` R`@_ \x84_R` Ra\x0E\x95a\x0E\x8F`@_ Ta!+V[\x15a\"\xAEV[\x01c\xFF\xFF\xFF\xFFa\x0E\xCEa\x0E\xA7\x83a\"[V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90a\"lV[\x16C\x11\x15a\x103Wa\x0E\xDF\x90a\"[V[`@Qc\tU\xF2\xD9`\xE4\x1B\x81R`\x04\x81\x01\x83\x90Rc\xFF\xFF\xFF\xFF\x91\x90\x91\x16`$\x82\x01R` \x81`D\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\r\x86W_\x91a\x10\x01W[P\x15a\x0F\xA2W_R`\x99` R`@_ \x90_R` R`@_ a\x0Ff\x81Ta!+V[`\x1F\x81\x11a\x0F\x82W[P`\x0Ef\x1C\xDB\x18\\\xDA\x19Y`\xCA\x1B\x01\x90U\0[a\x0F\x9C\x90\x82_R`\x1F` _ \x91\x01`\x05\x1C\x81\x01\x90a#\rV[\x81a\x0FoV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`1`$\x82\x01R\x7FOperator was not registered when`D\x82\x01Rp\x08\x1D\x18\\\xDA\xC8\x1D\xD8\\\xC8\x18\xDC\x99X]\x19Y`z\x1B`d\x82\x01R`\x84\x90\xFD[\x90P` \x81=` \x11a\x10+W[\x81a\x10\x1C` \x93\x83a\x1F\xCAV[\x81\x01\x03\x12a\r\x91WQ\x83a\x0FAV[=\x91Pa\x10\x0FV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FTask response time has not expir`D\x82\x01Re\x19Y\x08\x1EY]`\xD2\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FTask has already been responded `D\x82\x01Rato`\xF0\x1B`d\x82\x01R`\x84\x90\xFD[4a\r\x91W_6`\x03\x19\x01\x12a\r\x91W`3T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\r\x91W_6`\x03\x19\x01\x12a\r\x91W` c\xFF\xFF\xFF\xFF`\x97T\x16`@Q\x90\x81R\xF3[4a\r\x91Wa\x1106a ZV[a\x118a##V[Pa\x11Aa##V[\x90\x81Rc\xFF\xFF\xFF\xFFC\x16` \x82\x01R`@Q` \x81\x01\x90a\x11f\x81a\x0E/\x85\x85a \xB9V[Q\x90 c\xFF\xFF\xFF\xFF`\x97T\x16_R`\x98` R`@_ U`\x97T\x90`\x01c\xFF\xFF\xFF\xFF\x83\x16\x80\x7FX\x18\nj\x04\x03\xA6<+\\\xE4\xB8]\x12\x9DF\xA8\r7\x85\x1B\"\x16\xBD\n\x98\xB5\x9Es\t\xB8G`@Q\x80a\x11\xBB\x87\x82a \xB9V[\x03\x90\xA2\x01\x91c\xFF\xFF\xFF\xFF\x83\x11a\x11\xEDWc\xFF\xFF\xFF\xFFa\x05m\x93\x16\x90c\xFF\xFF\xFF\xFF\x19\x16\x17`\x97U`@Q\x91\x82\x91\x82a \xB9V[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[4a\r\x91W` 6`\x03\x19\x01\x12a\r\x91Wc\xFF\xFF\xFF\xFFa\x12\x1Fa\x1F6V[\x16_R`\x9A` R` `\xFF`@_ T\x16`@Q\x90\x15\x15\x81R\xF3[4a\r\x91W_6`\x03\x19\x01\x12a\r\x91W` `@Qc\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R\xF3[4a\r\x91W_6`\x03\x19\x01\x12a\r\x91Wa\x12\x93a#<V[`3\x80T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16\x90\x91U_\x90`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x82\x80\xA3\0[4a\r\x91W_6`\x03\x19\x01\x12a\r\x91W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\r\x91W_6`\x03\x19\x01\x12a\r\x91W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\r\x91W`@6`\x03\x19\x01\x12a\r\x91Wa\x13wa\x1E\xC1V[`$5\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\r\x91W_T\x91`\xFF\x83`\x08\x1C\x16\x15\x92\x83\x80\x94a\x14\xF3W[\x80\x15a\x14\xDCW[\x15a\x14\x80W`\xFF\x19\x81\x16`\x01\x17_U\x83a\x14oW[P`\xFF_T`\x08\x1C\x16\x15a\x14\x16Wa\x13\xD5a\x13\xDA\x92a%4V[a$\xD6V[a\x13\xE0W\0[a\xFF\0\x19_T\x16_U\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98` `@Q`\x01\x81R\xA1\0[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x90\xFD[a\xFF\xFF\x19\x16a\x01\x01\x17_U\x83a\x13\xBBV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x90\xFD[P0;\x15\x80\x15a\x13\xA6WP`\x01`\xFF\x82\x16\x14a\x13\xA6V[P`\x01`\xFF\x82\x16\x10a\x13\x9FV[4a\r\x91W` 6`\x03\x19\x01\x12a\r\x91Wa\r\x84a\x15\x1Ca\x1E\xC1V[a\x13\xD5a#<V[4a\r\x91W``6`\x03\x19\x01\x12a\r\x91W`\x045`\x01`\x01`@\x1B\x03\x81\x11a\r\x91W6\x81\x90\x03`\x04\x82\x01`@`\x03\x19\x83\x01\x12a\r\x91Wa\x15ba\x1FIV[\x90`D5`\x01`\x01`@\x1B\x03\x81\x11a\r\x91Wa\x15\x82\x906\x90`\x04\x01a <V[\x92a\x15\xA8c\xFF\xFF\xFF\xFF`@Q` \x81\x01\x90` \x82Ra\x0E=\x81a\x0E/`@\x82\x01\x89a!\xB4V[`$\x85\x01\x94c\xFF\xFF\xFF\xFFa\x15\xBEa\x0E\xA7\x88a\"[V[\x16C\x11a\x1B/W\x825\x91`\"\x19\x01\x82\x12\x15a\r\x91W\x01`\x04\x81\x015\x90`\x01`\x01`@\x1B\x03\x82\x11a\r\x91W`$\x01\x90\x806\x03\x82\x13a\r\x91Wa\x16,`'`@Q\x83\x81\x94` \x83\x01\x96f\x02C+ccya`\xCD\x1B\x88R\x84\x84\x017\x81\x01_\x83\x82\x01R\x03\x01`\x1F\x19\x81\x01\x83R\x82a\x1F\xCAV[Q\x90 \x7F\x19Ethereum Signed Message:\n32\0\0\0\0_R`\x1CR`<_ \x92\x80Q\x81\x01\x90``\x81` \x84\x01\x93\x03\x12a\r\x91W` \x81\x01Q`\x01`\x01`@\x1B\x03\x81\x11a\r\x91W\x81\x01\x95\x82`?\x88\x01\x12\x15a\r\x91W` \x87\x01Qa\x16\x9C\x81a!\x14V[\x97a\x16\xAA`@Q\x99\x8Aa\x1F\xCAV[\x81\x89R` \x80\x80\x8B\x01\x93`\x05\x1B\x83\x01\x01\x01\x90\x85\x82\x11a\r\x91W`@\x01\x91[\x81\x83\x10a\x1B\x0FWPPP`@\x82\x01Q`\x01`\x01`@\x1B\x03\x81\x11a\r\x91W\x82\x01\x92\x80`?\x85\x01\x12\x15a\r\x91W` \x84\x01Q\x93a\x17\x02\x85a!\x14V[\x94a\x17\x10`@Q\x96\x87a\x1F\xCAV[\x80\x86R` \x80\x80\x88\x01\x92`\x05\x1B\x84\x01\x01\x01\x91\x83\x83\x11a\r\x91W`@\x81\x01\x91[\x83\x83\x10a\x1A\xA5WPPPPP``\x82\x01Q\x90c\xFF\xFF\xFF\xFF\x82\x16\x80\x92\x03a\r\x91Wa\x17]c\xFF\xFF\xFF\xFF\x91a\"[V[\x16\x03a\x1AIW_[\x86Q\x81\x10\x15a\x192W`\x01`\x01`\xA0\x1B\x03a\x17\x80\x82\x89a\"\x86V[Q\x16_R`\x99` R`@_ \x85_R` Ra\x17\xA3a\x0E\x8F`@_ Ta!+V[a\x17\xAD\x81\x84a\"\x86V[Q`\x01`\x01`\xA0\x1B\x03a\x17\xC0\x83\x8Aa\"\x86V[Q\x16_R`\x99` R`@_ \x86_R` R`@_ \x90\x80Q\x90`\x01`\x01`@\x1B\x03\x82\x11a\r\xA4Wa\x17\xF3\x83Ta!+V[`\x1F\x81\x11a\x18\xF7W[P` \x90`\x1F\x83\x11`\x01\x14a\x18\x90W`\x01\x94\x93\x92\x91_\x91\x83a\x18\x85W[PP_\x19`\x03\x83\x90\x1B\x1C\x19\x16\x90\x84\x1B\x17\x90U[\x85\x7F\x8E\xB2\xD2\xFC\xCC\xF5\x80\x1E\x10\xFFX\xCDs\xE8x\x1B\xA9#\x12)cx\x93xw\x1F\x03\xC1\x14\x8B\x02>\x83\x80`\xA0\x1B\x03a\x18^\x84\x8Ca\"\x86V[Q\x16`@Q\x80\x91`@\x82Ra\x18v`@\x83\x01\x8Ba!\xB4V[\x90` \x83\x01R\x03\x90\xA2\x01a\x17eV[\x01Q\x90P\x8B\x80a\x18\x19V[\x90`\x1F\x19\x83\x16\x91\x84_R\x81_ \x92_[\x81\x81\x10a\x18\xDFWP\x91`\x01\x96\x95\x94\x92\x91\x83\x88\x95\x93\x10a\x18\xC7W[PPP\x81\x1B\x01\x90Ua\x18,V[\x01Q_\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U\x8B\x80\x80a\x18\xBAV[\x92\x93` `\x01\x81\x92\x87\x86\x01Q\x81U\x01\x95\x01\x93\x01a\x18\xA0V[a\x19\"\x90\x84_R` _ `\x1F\x85\x01`\x05\x1C\x81\x01\x91` \x86\x10a\x19(W[`\x1F\x01`\x05\x1C\x01\x90a#\rV[\x8Aa\x17\xFCV[\x90\x91P\x81\x90a\x19\x15V[a\x19w` \x83\x88\x88_R`\x9A\x83R`@_ `\x01`\xFF\x19\x82T\x16\x17\x90U`@Q\x93\x84\x92\x83\x92c\x0B\x13]?`\xE1\x1B\x84R`\x04\x84\x01R`@`$\x84\x01R`D\x83\x01\x90a \x95V[\x03\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\r\x86W_\x91a\x1A\x06W[P`\x01`\x01`\xE0\x1B\x03\x19\x16c\x0B\x13]?`\xE1\x1B\x03a\x19\xCDW\0[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01RpInvalid signature`x\x1B`D\x82\x01R`d\x90\xFD[\x90P` \x81=` \x11a\x1AAW[\x81a\x1A!` \x93\x83a\x1F\xCAV[\x81\x01\x03\x12a\r\x91WQ`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x03a\r\x91W\x81a\x19\xB3V[=\x91Pa\x1A\x14V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FReference block must match task `D\x82\x01Rmcreation block`\x90\x1B`d\x82\x01R`\x84\x90\xFD[\x82Q`\x01`\x01`@\x1B\x03\x81\x11a\r\x91W` \x90\x83\x01\x01\x85`?\x82\x01\x12\x15a\r\x91W` \x81\x01Q\x91a\x1A\xD5\x83a\x1F\xEBV[a\x1A\xE2`@Q\x91\x82a\x1F\xCAV[\x83\x81R`@\x83\x85\x01\x01\x88\x10a\r\x91W_` \x85\x81\x96`@\x83\x97\x01\x83\x86\x01^\x83\x01\x01R\x81R\x01\x92\x01\x91a\x17/V[\x82Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\r\x91W\x81R` \x92\x83\x01\x92\x01a\x16\xC8V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FTask response time has already e`D\x82\x01Re\x1E\x1C\x1A\\\x99Y`\xD2\x1B`d\x82\x01R`\x84\x90\xFD[4a\r\x91W` 6`\x03\x19\x01\x12a\r\x91Wa\x1B\x9Ca\x1E\xC1V[`@Qc\x02\xE0t\x03`\xE3\x1B\x81R_\x81`\x04\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\r\x86W_\x91a\x1E7W[P\x90\x81QQ\x90a\x1B\xF7\x82a!\x14V[\x90a\x1C\x05`@Q\x92\x83a\x1F\xCAV[\x82\x82Ra\x1C\x11\x83a!\x14V[` \x83\x01\x94\x90`\x1F\x19\x016\x867_[\x84\x81\x10a\x1E\tWPP`@\x80Qc\x90\x04\x13G`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`\x04\x83\x01R`$\x82\x01R\x81Q`D\x82\x01\x81\x90R\x90\x93\x84\x91`d\x83\x01\x91\x90_[\x81\x81\x10a\x1D\xE7WP_\x93\x92\x83\x90\x03\x91P\x82\x90P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x92\x83\x15a\r\x86W_\x93a\x1DUW[P__[\x83\x81\x10a\x1D)WPa\x1C\xC3\x90a$\xA4V[\x92_\x90_[\x84\x81\x10a\x1C\xDDW`@Q\x80a\x05m\x88\x82a\x1FmV[a\x1C\xE7\x81\x83a\"\x86V[Qa\x1C\xF5W[`\x01\x01a\x1C\xC8V[\x91`\x01\x90a\x1D!\x90`\x01`\x01`\xA0\x1B\x03a\x1D\x0F\x86\x88a\"\x86V[Q\x16a\x1D\x1B\x82\x8Aa\"\x86V[Ra$\x96V[\x92\x90Pa\x1C\xEDV[a\x1D3\x81\x86a\"\x86V[Qa\x1DAW[`\x01\x01a\x1C\xB2V[\x90a\x1DM`\x01\x91a$\x96V[\x91\x90Pa\x1D9V[\x90\x92P=\x80_\x83>a\x1Dg\x81\x83a\x1F\xCAV[\x81\x01\x90` \x81\x83\x03\x12a\r\x91W\x80Q\x90`\x01`\x01`@\x1B\x03\x82\x11a\r\x91W\x01\x90\x80`\x1F\x83\x01\x12\x15a\r\x91W\x81Qa\x1D\x9D\x81a!\x14V[\x92a\x1D\xAB`@Q\x94\x85a\x1F\xCAV[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\r\x91W` \x01\x90[\x82\x82\x10a\x1D\xD7WPPP\x91\x83a\x1C\xAEV[\x81Q\x81R` \x91\x82\x01\x91\x01a\x1D\xC6V[\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x84R\x87\x94P` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x1CaV[`\x01\x90\x81\x80`\xA0\x98\x95\x98\x1B\x03a\x1E \x82\x85Qa\"\x86V[QQ\x16a\x1E-\x82\x87a\"\x86V[R\x01\x94\x91\x94a\x1C V[a\x1EK\x91P=\x80_\x83>a\x05\x85\x81\x83a\x1F\xCAV[\x82a\x1B\xE8V[4a\r\x91W` 6`\x03\x19\x01\x12a\r\x91Wc\xFF\xFF\xFF\xFFa\x1Eoa\x1F6V[\x16_R`\x98` R` `@_ T`@Q\x90\x81R\xF3[4a\r\x91W` 6`\x03\x19\x01\x12a\r\x91Wa\x1E\x9Fa\x1E\xC1V[Pa\r\x84a#<V[4a\r\x91Wa\x1E\xB66a\x1E\xEBV[PPPa\r\x84a#<V[`\x045\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\r\x91WV[5\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\r\x91WV[``\x90`\x03\x19\x01\x12a\r\x91W`\x045`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\r\x91W\x90`$5`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\r\x91W\x90`D5`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x03a\r\x91W\x90V[`\x045\x90c\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\r\x91WV[`$5\x90c\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\r\x91WV[5\x90c\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\r\x91WV[` `@\x81\x83\x01\x92\x82\x81R\x84Q\x80\x94R\x01\x92\x01\x90_[\x81\x81\x10a\x1F\x90WPPP\x90V[\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x1F\x83V[`@\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\r\xA4W`@RV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\r\xA4W`@RV[`\x01`\x01`@\x1B\x03\x81\x11a\r\xA4W`\x1F\x01`\x1F\x19\x16` \x01\x90V[\x92\x91\x92a \x12\x82a\x1F\xEBV[\x91a  `@Q\x93\x84a\x1F\xCAV[\x82\x94\x81\x84R\x81\x83\x01\x11a\r\x91W\x82\x81` \x93\x84_\x96\x017\x01\x01RV[\x90\x80`\x1F\x83\x01\x12\x15a\r\x91W\x81` a W\x935\x91\x01a \x06V[\x90V[` `\x03\x19\x82\x01\x12a\r\x91W`\x045\x90`\x01`\x01`@\x1B\x03\x82\x11a\r\x91W\x80`#\x83\x01\x12\x15a\r\x91W\x81`$a W\x93`\x04\x015\x91\x01a \x06V[\x80Q\x80\x83R` \x92\x91\x81\x90\x84\x01\x84\x84\x01^_\x82\x82\x01\x84\x01R`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[` \x81R`@c\xFF\xFF\xFF\xFF` a \xDA\x85Q\x84\x83\x87\x01R``\x86\x01\x90a \x95V[\x94\x01Q\x16\x91\x01R\x90V[\x91\x81`\x1F\x84\x01\x12\x15a\r\x91W\x825\x91`\x01`\x01`@\x1B\x03\x83\x11a\r\x91W` \x80\x85\x01\x94\x84`\x05\x1B\x01\x01\x11a\r\x91WV[`\x01`\x01`@\x1B\x03\x81\x11a\r\xA4W`\x05\x1B` \x01\x90V[\x90`\x01\x82\x81\x1C\x92\x16\x80\x15a!YW[` \x83\x10\x14a!EWV[cNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[\x91`\x7F\x16\x91a!:V[\x905`\x1E\x19\x826\x03\x01\x81\x12\x15a\r\x91W\x01` \x815\x91\x01\x91`\x01`\x01`@\x1B\x03\x82\x11a\r\x91W\x816\x03\x83\x13a\r\x91WV[\x90\x80` \x93\x92\x81\x84R\x84\x84\x017_\x82\x82\x01\x84\x01R`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[\x90` c\xFF\xFF\xFF\xFFa!\xE2\x82a!\xDBa!\xCD\x87\x80a!cV[`@\x88R`@\x88\x01\x91a!\x94V[\x95\x01a\x1F\\V[\x16\x91\x01R\x90V[\x15a!\xF0WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`=`$\x82\x01R\x7Fsupplied task does not match the`D\x82\x01R\x7F one recorded in the contract\0\0\0`d\x82\x01R`\x84\x90\xFD[5c\xFF\xFF\xFF\xFF\x81\x16\x81\x03a\r\x91W\x90V[\x90c\xFF\xFF\xFF\xFF\x80\x91\x16\x91\x16\x01\x90c\xFF\xFF\xFF\xFF\x82\x11a\x11\xEDWV[\x80Q\x82\x10\x15a\"\x9AW` \x91`\x05\x1B\x01\x01\x90V[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[\x15a\"\xB5WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FOperator has already responded t`D\x82\x01Rio the task`\xB0\x1B`d\x82\x01R`\x84\x90\xFD[\x81\x81\x10a#\x18WPPV[_\x81U`\x01\x01a#\rV[`@Q\x90a#0\x82a\x1F\xAFV[_` \x83``\x81R\x01RV[`3T`\x01`\x01`\xA0\x1B\x03\x163\x03a#PWV[`d`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R` `$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R\xFD[` \x81\x83\x03\x12a\r\x91W\x80Q\x90`\x01`\x01`@\x1B\x03\x82\x11a\r\x91W\x01\x90` \x82\x82\x03\x12a\r\x91W`@Q\x91` \x83\x01\x83\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\r\xA4W`@R\x80Q\x90`\x01`\x01`@\x1B\x03\x82\x11a\r\x91W\x01\x81`\x1F\x82\x01\x12\x15a\r\x91W\x80Q\x90a$\x01\x82a!\x14V[\x92a$\x0F`@Q\x94\x85a\x1F\xCAV[\x82\x84R` \x80\x85\x01\x93`\x06\x1B\x83\x01\x01\x91\x81\x83\x11a\r\x91W` \x01\x92[\x82\x84\x10a$;WPPPP\x81R\x90V[`@\x84\x83\x03\x12a\r\x91W`@Q\x90a$R\x82a\x1F\xAFV[\x84Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\r\x91W\x82R` \x85\x01Q\x90k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\r\x91W\x82` \x92\x83`@\x95\x01R\x81R\x01\x93\x01\x92a$+V[_\x19\x81\x14a\x11\xEDW`\x01\x01\x90V[\x90a$\xAE\x82a!\x14V[a$\xBB`@Q\x91\x82a\x1F\xCAV[\x82\x81R\x80\x92a$\xCC`\x1F\x19\x91a!\x14V[\x01\x90` 6\x91\x017V[`eT`@\x80Q`\x01`\x01`\xA0\x1B\x03\x80\x84\x16\x82R\x84\x16` \x82\x01R\x91\x92\x91\x7F\xE1\x1C\xDD\xF1\x81jC1\x8C\xA1u\xBB\xC5,\xD0\x18T6\xE9\xCB\xEA\xD7\xC8:\xCCT\xA7>F\x17\x17\xE3\x91\x90\xA1`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x19\x91\x90\x91\x16\x17`eUV[`3\x80T`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x01`\x01`\xA0\x1B\x03\x19\x82\x16\x81\x17\x90\x92U\x90\x91\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0_\x80\xA3V[`eT`\x01`\x01`\xA0\x1B\x03\x163\x03a%\x90WV[c\x8Ey\xFD\xB5`\xE0\x1B_R`\x04_\xFD[\x91\x90\x81\x10\x15a\"\x9AW`\x05\x1B\x81\x015\x90`\xBE\x19\x816\x03\x01\x82\x12\x15a\r\x91W\x01\x90V[\x905\x90`\x1E\x19\x816\x03\x01\x82\x12\x15a\r\x91W\x01\x805\x90`\x01`\x01`@\x1B\x03\x82\x11a\r\x91W` \x01\x91\x81`\x06\x1B6\x03\x83\x13a\r\x91WV[\x91\x90\x82\x01\x80\x92\x11a\x11\xEDWV[5`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\r\x91W\x90V[\x905`\x1E\x19\x826\x03\x01\x81\x12\x15a\r\x91W\x01` \x815\x91\x01\x91`\x01`\x01`@\x1B\x03\x82\x11a\r\x91W\x81`\x06\x1B6\x03\x83\x13a\r\x91WV[\x91` \x90\x82\x81R\x01\x91\x90_\x90[\x80\x82\x10a&eWPPP\x90V[\x90\x91\x92\x835`\x01\x80`\xA0\x1B\x03\x81\x16\x80\x91\x03a\r\x91W\x81R` \x84\x015\x90k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x80\x92\x03a\r\x91W`@\x81`\x01\x93` \x83\x94\x01R\x01\x94\x01\x92\x01\x90a&XV[\x91\x90\x81\x10\x15a\"\x9AW`\x05\x1B\x81\x015\x90`\x9E\x19\x816\x03\x01\x82\x12\x15a\r\x91W\x01\x90V[`@Qc#\xB8r\xDD`\xE0\x1B` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`$\x82\x01R\x92\x90\x91\x16`D\x83\x01R`d\x80\x83\x01\x93\x90\x93R\x91\x81Ra'\x1A\x91a'\x15`\x84\x83a\x1F\xCAV[a'\xDAV[V[`@Qcn\xB1v\x9F`\xE1\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16`$\x82\x01R` \x81\x80`D\x81\x01\x03\x81`\x01`\x01`\xA0\x1B\x03\x86\x16Z\xFA\x90\x81\x15a\r\x86W_\x91a'\xA6W[Pa'\x1A\x93a'p\x91a%\xF6V[`@Qc\t^\xA7\xB3`\xE0\x1B` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x93\x16`$\x84\x01R`D\x80\x84\x01\x91\x90\x91R\x82Ra'\x15`d\x83a\x1F\xCAV[\x90P` \x81=` \x11a'\xD2W[\x81a'\xC1` \x93\x83a\x1F\xCAV[\x81\x01\x03\x12a\r\x91WQa'\x1Aa'bV[=\x91Pa'\xB4V[\x90a(Y\x91`\x01\x80`\xA0\x1B\x03\x16_\x80`@Q\x93a'\xF8`@\x86a\x1F\xCAV[` \x85R\x7FSafeERC20: low-level call failed` \x86\x01R` \x81Q\x91\x01\x82\x85Z\xF1=\x15a(\xEAW=\x91a(=\x83a\x1F\xEBV[\x92a(K`@Q\x94\x85a\x1F\xCAV[\x83R=_` \x85\x01>a(\xEEV[\x80Q\x90\x81\x15\x91\x82\x15a(\xC7W[PP\x15a(oWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FSafeERC20: ERC20 operation did n`D\x82\x01Ri\x1B\xDD\x08\x1C\xDDX\xD8\xD9YY`\xB2\x1B`d\x82\x01R`\x84\x90\xFD[\x81\x92P\x90` \x91\x81\x01\x03\x12a\r\x91W` \x01Q\x80\x15\x15\x81\x03a\r\x91W_\x80a(fV[``\x91[\x91\x92\x90\x15a)PWP\x81Q\x15a)\x02WP\x90V[;\x15a)\x0BW\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R`d\x90\xFD[\x82Q\x90\x91P\x15a)cWP\x80Q\x90` \x01\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R\x90\x81\x90a)\x87\x90`$\x83\x01\x90a \x95V[\x03\x90\xFD\xFE\xA2dipfsX\"\x12 ;TcB\xEF!\xA7%]\r{t\x11\x81\x8Ce\xD3\x8E\xDB\xCD\xFC\x1Ex\x86)\x16\x9D\x92\xE5\xBA\x18\xBCdsolcC\0\x08\x1B\x003",
    );
    /**Custom error with signature `DelayPeriodNotPassed()` and selector `0xfb623b04`.
    ```solidity
    error DelayPeriodNotPassed();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct DelayPeriodNotPassed {}
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
        impl ::core::convert::From<DelayPeriodNotPassed> for UnderlyingRustTuple<'_> {
            fn from(value: DelayPeriodNotPassed) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for DelayPeriodNotPassed {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for DelayPeriodNotPassed {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "DelayPeriodNotPassed()";
            const SELECTOR: [u8; 4] = [251u8, 98u8, 59u8, 4u8];
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
    /**Custom error with signature `OnlyRegistryCoordinator()` and selector `0x8729b7be`.
    ```solidity
    error OnlyRegistryCoordinator();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OnlyRegistryCoordinator {}
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
        impl ::core::convert::From<OnlyRegistryCoordinator> for UnderlyingRustTuple<'_> {
            fn from(value: OnlyRegistryCoordinator) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OnlyRegistryCoordinator {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OnlyRegistryCoordinator {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OnlyRegistryCoordinator()";
            const SELECTOR: [u8; 4] = [135u8, 41u8, 183u8, 190u8];
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
    /**Custom error with signature `OnlyRewardsInitiator()` and selector `0x8e79fdb5`.
    ```solidity
    error OnlyRewardsInitiator();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OnlyRewardsInitiator {}
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
        impl ::core::convert::From<OnlyRewardsInitiator> for UnderlyingRustTuple<'_> {
            fn from(value: OnlyRewardsInitiator) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OnlyRewardsInitiator {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OnlyRewardsInitiator {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OnlyRewardsInitiator()";
            const SELECTOR: [u8; 4] = [142u8, 121u8, 253u8, 181u8];
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
    /**Custom error with signature `OnlyStakeRegistry()` and selector `0x46bf2281`.
    ```solidity
    error OnlyStakeRegistry();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OnlyStakeRegistry {}
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
        impl ::core::convert::From<OnlyStakeRegistry> for UnderlyingRustTuple<'_> {
            fn from(value: OnlyStakeRegistry) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OnlyStakeRegistry {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OnlyStakeRegistry {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OnlyStakeRegistry()";
            const SELECTOR: [u8; 4] = [70u8, 191u8, 34u8, 129u8];
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
    /**Event with signature `NewTaskCreated(uint32,(string,uint32))` and selector `0x58180a6a0403a63c2b5ce4b85d129d46a80d37851b2216bd0a98b59e7309b847`.
    ```solidity
    event NewTaskCreated(uint32 indexed taskIndex, IHelloWorldServiceManager.Task task);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct NewTaskCreated {
        #[allow(missing_docs)]
        pub taskIndex: u32,
        #[allow(missing_docs)]
        pub task: <IHelloWorldServiceManager::Task as alloy::sol_types::SolType>::RustType,
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
        impl alloy_sol_types::SolEvent for NewTaskCreated {
            type DataTuple<'a> = (IHelloWorldServiceManager::Task,);
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<32>,
            );
            const SIGNATURE: &'static str = "NewTaskCreated(uint32,(string,uint32))";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    88u8, 24u8, 10u8, 106u8, 4u8, 3u8, 166u8, 60u8, 43u8, 92u8, 228u8, 184u8, 93u8,
                    18u8, 157u8, 70u8, 168u8, 13u8, 55u8, 133u8, 27u8, 34u8, 22u8, 189u8, 10u8,
                    152u8, 181u8, 158u8, 115u8, 9u8, 184u8, 71u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    taskIndex: topics.1,
                    task: data.0,
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
                    <IHelloWorldServiceManager::Task as alloy_sol_types::SolType>::tokenize(
                        &self.task,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.taskIndex.clone())
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
                out[1usize] = <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.taskIndex);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for NewTaskCreated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&NewTaskCreated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &NewTaskCreated) -> alloy_sol_types::private::LogData {
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
    /**Event with signature `RewardsInitiatorUpdated(address,address)` and selector `0xe11cddf1816a43318ca175bbc52cd0185436e9cbead7c83acc54a73e461717e3`.
    ```solidity
    event RewardsInitiatorUpdated(address prevRewardsInitiator, address newRewardsInitiator);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct RewardsInitiatorUpdated {
        #[allow(missing_docs)]
        pub prevRewardsInitiator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub newRewardsInitiator: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for RewardsInitiatorUpdated {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "RewardsInitiatorUpdated(address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    225u8, 28u8, 221u8, 241u8, 129u8, 106u8, 67u8, 49u8, 140u8, 161u8, 117u8,
                    187u8, 197u8, 44u8, 208u8, 24u8, 84u8, 54u8, 233u8, 203u8, 234u8, 215u8, 200u8,
                    58u8, 204u8, 84u8, 167u8, 62u8, 70u8, 23u8, 23u8, 227u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    prevRewardsInitiator: data.0,
                    newRewardsInitiator: data.1,
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
                        &self.prevRewardsInitiator,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.newRewardsInitiator,
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
        impl alloy_sol_types::private::IntoLogData for RewardsInitiatorUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&RewardsInitiatorUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &RewardsInitiatorUpdated) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `TaskResponded(uint32,(string,uint32),address)` and selector `0x8eb2d2fcccf5801e10ff58cd73e8781ba923122963789378771f03c1148b023e`.
    ```solidity
    event TaskResponded(uint32 indexed taskIndex, IHelloWorldServiceManager.Task task, address operator);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct TaskResponded {
        #[allow(missing_docs)]
        pub taskIndex: u32,
        #[allow(missing_docs)]
        pub task: <IHelloWorldServiceManager::Task as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for TaskResponded {
            type DataTuple<'a> = (
                IHelloWorldServiceManager::Task,
                alloy::sol_types::sol_data::Address,
            );
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<32>,
            );
            const SIGNATURE: &'static str = "TaskResponded(uint32,(string,uint32),address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    142u8, 178u8, 210u8, 252u8, 204u8, 245u8, 128u8, 30u8, 16u8, 255u8, 88u8,
                    205u8, 115u8, 232u8, 120u8, 27u8, 169u8, 35u8, 18u8, 41u8, 99u8, 120u8, 147u8,
                    120u8, 119u8, 31u8, 3u8, 193u8, 20u8, 139u8, 2u8, 62u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    taskIndex: topics.1,
                    task: data.0,
                    operator: data.1,
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
                    <IHelloWorldServiceManager::Task as alloy_sol_types::SolType>::tokenize(
                        &self.task,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.operator,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.taskIndex.clone())
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
                out[1usize] = <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.taskIndex);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for TaskResponded {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&TaskResponded> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &TaskResponded) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Constructor`.
    ```solidity
    constructor(address _avsDirectory, address _stakeRegistry, address _rewardsCoordinator, address _delegationManager, address _allocationManager, uint32 _maxResponseIntervalBlocks);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct constructorCall {
        #[allow(missing_docs)]
        pub _avsDirectory: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _stakeRegistry: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _rewardsCoordinator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _delegationManager: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _allocationManager: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _maxResponseIntervalBlocks: u32,
    }
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                u32,
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
            impl ::core::convert::From<constructorCall> for UnderlyingRustTuple<'_> {
                fn from(value: constructorCall) -> Self {
                    (
                        value._avsDirectory,
                        value._stakeRegistry,
                        value._rewardsCoordinator,
                        value._delegationManager,
                        value._allocationManager,
                        value._maxResponseIntervalBlocks,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for constructorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _avsDirectory: tuple.0,
                        _stakeRegistry: tuple.1,
                        _rewardsCoordinator: tuple.2,
                        _delegationManager: tuple.3,
                        _allocationManager: tuple.4,
                        _maxResponseIntervalBlocks: tuple.5,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolConstructor for constructorCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<32>,
            );
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
                        &self._avsDirectory,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._stakeRegistry,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._rewardsCoordinator,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._delegationManager,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._allocationManager,
                    ),
                    <alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::SolType>::tokenize(
                        &self._maxResponseIntervalBlocks,
                    ),
                )
            }
        }
    };
    /**Function with signature `MAX_RESPONSE_INTERVAL_BLOCKS()` and selector `0x733d056c`.
    ```solidity
    function MAX_RESPONSE_INTERVAL_BLOCKS() external view returns (uint32);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct MAX_RESPONSE_INTERVAL_BLOCKSCall {}
    ///Container type for the return parameters of the [`MAX_RESPONSE_INTERVAL_BLOCKS()`](MAX_RESPONSE_INTERVAL_BLOCKSCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct MAX_RESPONSE_INTERVAL_BLOCKSReturn {
        #[allow(missing_docs)]
        pub _0: u32,
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
            impl ::core::convert::From<MAX_RESPONSE_INTERVAL_BLOCKSCall> for UnderlyingRustTuple<'_> {
                fn from(value: MAX_RESPONSE_INTERVAL_BLOCKSCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for MAX_RESPONSE_INTERVAL_BLOCKSCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
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
            impl ::core::convert::From<MAX_RESPONSE_INTERVAL_BLOCKSReturn> for UnderlyingRustTuple<'_> {
                fn from(value: MAX_RESPONSE_INTERVAL_BLOCKSReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for MAX_RESPONSE_INTERVAL_BLOCKSReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for MAX_RESPONSE_INTERVAL_BLOCKSCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = MAX_RESPONSE_INTERVAL_BLOCKSReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "MAX_RESPONSE_INTERVAL_BLOCKS()";
            const SELECTOR: [u8; 4] = [115u8, 61u8, 5u8, 108u8];
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
    /**Function with signature `addPendingAdmin(address)` and selector `0x279432eb`.
    ```solidity
    function addPendingAdmin(address admin) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct addPendingAdminCall {
        #[allow(missing_docs)]
        pub admin: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`addPendingAdmin(address)`](addPendingAdminCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct addPendingAdminReturn {}
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
            impl ::core::convert::From<addPendingAdminCall> for UnderlyingRustTuple<'_> {
                fn from(value: addPendingAdminCall) -> Self {
                    (value.admin,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for addPendingAdminCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { admin: tuple.0 }
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
            impl ::core::convert::From<addPendingAdminReturn> for UnderlyingRustTuple<'_> {
                fn from(value: addPendingAdminReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for addPendingAdminReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for addPendingAdminCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = addPendingAdminReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "addPendingAdmin(address)";
            const SELECTOR: [u8; 4] = [39u8, 148u8, 50u8, 235u8];
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
                        &self.admin,
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
    /**Function with signature `allTaskHashes(uint32)` and selector `0x2d89f6fc`.
    ```solidity
    function allTaskHashes(uint32) external view returns (bytes32);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct allTaskHashesCall {
        #[allow(missing_docs)]
        pub _0: u32,
    }
    ///Container type for the return parameters of the [`allTaskHashes(uint32)`](allTaskHashesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct allTaskHashesReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::FixedBytes<32>,
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
            impl ::core::convert::From<allTaskHashesCall> for UnderlyingRustTuple<'_> {
                fn from(value: allTaskHashesCall) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for allTaskHashesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
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
            impl ::core::convert::From<allTaskHashesReturn> for UnderlyingRustTuple<'_> {
                fn from(value: allTaskHashesReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for allTaskHashesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for allTaskHashesCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = allTaskHashesReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "allTaskHashes(uint32)";
            const SELECTOR: [u8; 4] = [45u8, 137u8, 246u8, 252u8];
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
                        &self._0,
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
    /**Function with signature `allTaskResponses(address,uint32)` and selector `0xc20bab7f`.
    ```solidity
    function allTaskResponses(address, uint32) external view returns (bytes memory);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct allTaskResponsesCall {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _1: u32,
    }
    ///Container type for the return parameters of the [`allTaskResponses(address,uint32)`](allTaskResponsesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct allTaskResponsesReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Bytes,
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
            impl ::core::convert::From<allTaskResponsesCall> for UnderlyingRustTuple<'_> {
                fn from(value: allTaskResponsesCall) -> Self {
                    (value._0, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for allTaskResponsesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _0: tuple.0,
                        _1: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Bytes,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Bytes,);
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
            impl ::core::convert::From<allTaskResponsesReturn> for UnderlyingRustTuple<'_> {
                fn from(value: allTaskResponsesReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for allTaskResponsesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for allTaskResponsesCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<32>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = allTaskResponsesReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bytes,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "allTaskResponses(address,uint32)";
            const SELECTOR: [u8; 4] = [194u8, 11u8, 171u8, 127u8];
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
                        &self._0,
                    ),
                    <alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::SolType>::tokenize(
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
    /**Function with signature `allocationManager()` and selector `0xca8aa7c7`.
    ```solidity
    function allocationManager() external view returns (address);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct allocationManagerCall {}
    ///Container type for the return parameters of the [`allocationManager()`](allocationManagerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct allocationManagerReturn {
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
            impl ::core::convert::From<allocationManagerCall> for UnderlyingRustTuple<'_> {
                fn from(value: allocationManagerCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for allocationManagerCall {
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
            impl ::core::convert::From<allocationManagerReturn> for UnderlyingRustTuple<'_> {
                fn from(value: allocationManagerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for allocationManagerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for allocationManagerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = allocationManagerReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "allocationManager()";
            const SELECTOR: [u8; 4] = [202u8, 138u8, 167u8, 199u8];
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
    /**Function with signature `avsDirectory()` and selector `0x6b3aa72e`.
    ```solidity
    function avsDirectory() external view returns (address);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct avsDirectoryCall {}
    ///Container type for the return parameters of the [`avsDirectory()`](avsDirectoryCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct avsDirectoryReturn {
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
            impl ::core::convert::From<avsDirectoryCall> for UnderlyingRustTuple<'_> {
                fn from(value: avsDirectoryCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for avsDirectoryCall {
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
            impl ::core::convert::From<avsDirectoryReturn> for UnderlyingRustTuple<'_> {
                fn from(value: avsDirectoryReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for avsDirectoryReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for avsDirectoryCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = avsDirectoryReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "avsDirectory()";
            const SELECTOR: [u8; 4] = [107u8, 58u8, 167u8, 46u8];
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
    /**Function with signature `createAVSRewardsSubmission(((address,uint96)[],address,uint256,uint32,uint32)[])` and selector `0xfce36c7d`.
    ```solidity
    function createAVSRewardsSubmission(IRewardsCoordinatorTypes.RewardsSubmission[] memory rewardsSubmissions) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct createAVSRewardsSubmissionCall {
        #[allow(missing_docs)]
        pub rewardsSubmissions: alloy::sol_types::private::Vec<
            <IRewardsCoordinatorTypes::RewardsSubmission as alloy::sol_types::SolType>::RustType,
        >,
    }
    ///Container type for the return parameters of the [`createAVSRewardsSubmission(((address,uint96)[],address,uint256,uint32,uint32)[])`](createAVSRewardsSubmissionCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct createAVSRewardsSubmissionReturn {}
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
                (alloy::sol_types::sol_data::Array<IRewardsCoordinatorTypes::RewardsSubmission>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <IRewardsCoordinatorTypes::RewardsSubmission as alloy::sol_types::SolType>::RustType,
                >,
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
            impl ::core::convert::From<createAVSRewardsSubmissionCall> for UnderlyingRustTuple<'_> {
                fn from(value: createAVSRewardsSubmissionCall) -> Self {
                    (value.rewardsSubmissions,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for createAVSRewardsSubmissionCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        rewardsSubmissions: tuple.0,
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
            impl ::core::convert::From<createAVSRewardsSubmissionReturn> for UnderlyingRustTuple<'_> {
                fn from(value: createAVSRewardsSubmissionReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for createAVSRewardsSubmissionReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for createAVSRewardsSubmissionCall {
            type Parameters<'a> =
                (alloy::sol_types::sol_data::Array<IRewardsCoordinatorTypes::RewardsSubmission>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = createAVSRewardsSubmissionReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str =
                "createAVSRewardsSubmission(((address,uint96)[],address,uint256,uint32,uint32)[])";
            const SELECTOR: [u8; 4] = [252u8, 227u8, 108u8, 125u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (<alloy::sol_types::sol_data::Array<
                    IRewardsCoordinatorTypes::RewardsSubmission,
                > as alloy_sol_types::SolType>::tokenize(
                    &self.rewardsSubmissions,
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
    /**Function with signature `createNewTask(string)` and selector `0x85edf874`.
    ```solidity
    function createNewTask(string memory name) external returns (IHelloWorldServiceManager.Task memory);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct createNewTaskCall {
        #[allow(missing_docs)]
        pub name: alloy::sol_types::private::String,
    }
    ///Container type for the return parameters of the [`createNewTask(string)`](createNewTaskCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct createNewTaskReturn {
        #[allow(missing_docs)]
        pub _0: <IHelloWorldServiceManager::Task as alloy::sol_types::SolType>::RustType,
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
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::String,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::String,);
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
            impl ::core::convert::From<createNewTaskCall> for UnderlyingRustTuple<'_> {
                fn from(value: createNewTaskCall) -> Self {
                    (value.name,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for createNewTaskCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { name: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (IHelloWorldServiceManager::Task,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> =
                (<IHelloWorldServiceManager::Task as alloy::sol_types::SolType>::RustType,);
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
            impl ::core::convert::From<createNewTaskReturn> for UnderlyingRustTuple<'_> {
                fn from(value: createNewTaskReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for createNewTaskReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for createNewTaskCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::String,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = createNewTaskReturn;
            type ReturnTuple<'a> = (IHelloWorldServiceManager::Task,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "createNewTask(string)";
            const SELECTOR: [u8; 4] = [133u8, 237u8, 248u8, 116u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.name,
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
    /**Function with signature `createOperatorDirectedAVSRewardsSubmission(((address,uint96)[],address,(address,uint256)[],uint32,uint32,string)[])` and selector `0xa20b99bf`.
    ```solidity
    function createOperatorDirectedAVSRewardsSubmission(IRewardsCoordinatorTypes.OperatorDirectedRewardsSubmission[] memory operatorDirectedRewardsSubmissions) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct createOperatorDirectedAVSRewardsSubmissionCall {
        #[allow(missing_docs)]
        pub operatorDirectedRewardsSubmissions: alloy::sol_types::private::Vec<
            <IRewardsCoordinatorTypes::OperatorDirectedRewardsSubmission as alloy::sol_types::SolType>::RustType,
        >,
    }
    ///Container type for the return parameters of the [`createOperatorDirectedAVSRewardsSubmission(((address,uint96)[],address,(address,uint256)[],uint32,uint32,string)[])`](createOperatorDirectedAVSRewardsSubmissionCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct createOperatorDirectedAVSRewardsSubmissionReturn {}
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
                    IRewardsCoordinatorTypes::OperatorDirectedRewardsSubmission,
                >,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <IRewardsCoordinatorTypes::OperatorDirectedRewardsSubmission as alloy::sol_types::SolType>::RustType,
                >,
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
            impl ::core::convert::From<createOperatorDirectedAVSRewardsSubmissionCall>
                for UnderlyingRustTuple<'_>
            {
                fn from(value: createOperatorDirectedAVSRewardsSubmissionCall) -> Self {
                    (value.operatorDirectedRewardsSubmissions,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
                for createOperatorDirectedAVSRewardsSubmissionCall
            {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operatorDirectedRewardsSubmissions: tuple.0,
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
            impl ::core::convert::From<createOperatorDirectedAVSRewardsSubmissionReturn>
                for UnderlyingRustTuple<'_>
            {
                fn from(value: createOperatorDirectedAVSRewardsSubmissionReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
                for createOperatorDirectedAVSRewardsSubmissionReturn
            {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for createOperatorDirectedAVSRewardsSubmissionCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<
                    IRewardsCoordinatorTypes::OperatorDirectedRewardsSubmission,
                >,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = createOperatorDirectedAVSRewardsSubmissionReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "createOperatorDirectedAVSRewardsSubmission(((address,uint96)[],address,(address,uint256)[],uint32,uint32,string)[])";
            const SELECTOR: [u8; 4] = [162u8, 11u8, 153u8, 191u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (<alloy::sol_types::sol_data::Array<
                    IRewardsCoordinatorTypes::OperatorDirectedRewardsSubmission,
                > as alloy_sol_types::SolType>::tokenize(
                    &self.operatorDirectedRewardsSubmissions,
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
    /**Function with signature `deregisterOperatorFromAVS(address)` and selector `0xa364f4da`.
    ```solidity
    function deregisterOperatorFromAVS(address operator) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct deregisterOperatorFromAVSCall {
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`deregisterOperatorFromAVS(address)`](deregisterOperatorFromAVSCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct deregisterOperatorFromAVSReturn {}
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
            impl ::core::convert::From<deregisterOperatorFromAVSCall> for UnderlyingRustTuple<'_> {
                fn from(value: deregisterOperatorFromAVSCall) -> Self {
                    (value.operator,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for deregisterOperatorFromAVSCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { operator: tuple.0 }
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
            impl ::core::convert::From<deregisterOperatorFromAVSReturn> for UnderlyingRustTuple<'_> {
                fn from(value: deregisterOperatorFromAVSReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for deregisterOperatorFromAVSReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for deregisterOperatorFromAVSCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = deregisterOperatorFromAVSReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "deregisterOperatorFromAVS(address)";
            const SELECTOR: [u8; 4] = [163u8, 100u8, 244u8, 218u8];
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
    /**Function with signature `deregisterOperatorFromOperatorSets(address,uint32[])` and selector `0xc1a8e2c5`.
    ```solidity
    function deregisterOperatorFromOperatorSets(address operator, uint32[] memory operatorSetIds) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct deregisterOperatorFromOperatorSetsCall {
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub operatorSetIds: alloy::sol_types::private::Vec<u32>,
    }
    ///Container type for the return parameters of the [`deregisterOperatorFromOperatorSets(address,uint32[])`](deregisterOperatorFromOperatorSetsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct deregisterOperatorFromOperatorSetsReturn {}
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
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<32>>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Vec<u32>,
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
            impl ::core::convert::From<deregisterOperatorFromOperatorSetsCall> for UnderlyingRustTuple<'_> {
                fn from(value: deregisterOperatorFromOperatorSetsCall) -> Self {
                    (value.operator, value.operatorSetIds)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for deregisterOperatorFromOperatorSetsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operator: tuple.0,
                        operatorSetIds: tuple.1,
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
            impl ::core::convert::From<deregisterOperatorFromOperatorSetsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: deregisterOperatorFromOperatorSetsReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for deregisterOperatorFromOperatorSetsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for deregisterOperatorFromOperatorSetsCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<32>>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = deregisterOperatorFromOperatorSetsReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "deregisterOperatorFromOperatorSets(address,uint32[])";
            const SELECTOR: [u8; 4] = [193u8, 168u8, 226u8, 197u8];
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
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<32>,
                    > as alloy_sol_types::SolType>::tokenize(&self.operatorSetIds),
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
    /**Function with signature `getOperatorRestakedStrategies(address)` and selector `0x33cfb7b7`.
    ```solidity
    function getOperatorRestakedStrategies(address _operator) external view returns (address[] memory);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorRestakedStrategiesCall {
        #[allow(missing_docs)]
        pub _operator: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`getOperatorRestakedStrategies(address)`](getOperatorRestakedStrategiesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorRestakedStrategiesReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
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
            impl ::core::convert::From<getOperatorRestakedStrategiesCall> for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorRestakedStrategiesCall) -> Self {
                    (value._operator,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getOperatorRestakedStrategiesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _operator: tuple.0 }
                }
            }
        }
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
            impl ::core::convert::From<getOperatorRestakedStrategiesReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorRestakedStrategiesReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getOperatorRestakedStrategiesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getOperatorRestakedStrategiesCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getOperatorRestakedStrategiesReturn;
            type ReturnTuple<'a> =
                (alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getOperatorRestakedStrategies(address)";
            const SELECTOR: [u8; 4] = [51u8, 207u8, 183u8, 183u8];
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
                        &self._operator,
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
    /**Function with signature `getRestakeableStrategies()` and selector `0xe481af9d`.
    ```solidity
    function getRestakeableStrategies() external view returns (address[] memory);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getRestakeableStrategiesCall {}
    ///Container type for the return parameters of the [`getRestakeableStrategies()`](getRestakeableStrategiesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getRestakeableStrategiesReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
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
            impl ::core::convert::From<getRestakeableStrategiesCall> for UnderlyingRustTuple<'_> {
                fn from(value: getRestakeableStrategiesCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getRestakeableStrategiesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
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
            impl ::core::convert::From<getRestakeableStrategiesReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getRestakeableStrategiesReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getRestakeableStrategiesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getRestakeableStrategiesCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getRestakeableStrategiesReturn;
            type ReturnTuple<'a> =
                (alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getRestakeableStrategies()";
            const SELECTOR: [u8; 4] = [228u8, 129u8, 175u8, 157u8];
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
    /**Function with signature `initialize(address,address)` and selector `0x485cc955`.
    ```solidity
    function initialize(address initialOwner, address _rewardsInitiator) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initializeCall {
        #[allow(missing_docs)]
        pub initialOwner: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _rewardsInitiator: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`initialize(address,address)`](initializeCall) function.
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
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
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
            impl ::core::convert::From<initializeCall> for UnderlyingRustTuple<'_> {
                fn from(value: initializeCall) -> Self {
                    (value.initialOwner, value._rewardsInitiator)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for initializeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        initialOwner: tuple.0,
                        _rewardsInitiator: tuple.1,
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
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = initializeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "initialize(address,address)";
            const SELECTOR: [u8; 4] = [72u8, 92u8, 201u8, 85u8];
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
                        &self.initialOwner,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._rewardsInitiator,
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
    /**Function with signature `latestTaskNum()` and selector `0x8b00ce7c`.
    ```solidity
    function latestTaskNum() external view returns (uint32);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct latestTaskNumCall {}
    ///Container type for the return parameters of the [`latestTaskNum()`](latestTaskNumCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct latestTaskNumReturn {
        #[allow(missing_docs)]
        pub _0: u32,
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
            impl ::core::convert::From<latestTaskNumCall> for UnderlyingRustTuple<'_> {
                fn from(value: latestTaskNumCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for latestTaskNumCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
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
            impl ::core::convert::From<latestTaskNumReturn> for UnderlyingRustTuple<'_> {
                fn from(value: latestTaskNumReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for latestTaskNumReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for latestTaskNumCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = latestTaskNumReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "latestTaskNum()";
            const SELECTOR: [u8; 4] = [139u8, 0u8, 206u8, 124u8];
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
    /**Function with signature `registerOperatorToAVS(address,(bytes,bytes32,uint256))` and selector `0x9926ee7d`.
    ```solidity
    function registerOperatorToAVS(address operator, ISignatureUtilsMixinTypes.SignatureWithSaltAndExpiry memory operatorSignature) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct registerOperatorToAVSCall {
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub operatorSignature: <ISignatureUtilsMixinTypes::SignatureWithSaltAndExpiry as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`registerOperatorToAVS(address,(bytes,bytes32,uint256))`](registerOperatorToAVSCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct registerOperatorToAVSReturn {}
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
                ISignatureUtilsMixinTypes::SignatureWithSaltAndExpiry,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                <ISignatureUtilsMixinTypes::SignatureWithSaltAndExpiry as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<registerOperatorToAVSCall> for UnderlyingRustTuple<'_> {
                fn from(value: registerOperatorToAVSCall) -> Self {
                    (value.operator, value.operatorSignature)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for registerOperatorToAVSCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operator: tuple.0,
                        operatorSignature: tuple.1,
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
            impl ::core::convert::From<registerOperatorToAVSReturn> for UnderlyingRustTuple<'_> {
                fn from(value: registerOperatorToAVSReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for registerOperatorToAVSReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for registerOperatorToAVSCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                ISignatureUtilsMixinTypes::SignatureWithSaltAndExpiry,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = registerOperatorToAVSReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str =
                "registerOperatorToAVS(address,(bytes,bytes32,uint256))";
            const SELECTOR: [u8; 4] = [153u8, 38u8, 238u8, 125u8];
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
                    <ISignatureUtilsMixinTypes::SignatureWithSaltAndExpiry as alloy_sol_types::SolType>::tokenize(
                        &self.operatorSignature,
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
    /**Function with signature `removeAdmin(address)` and selector `0x1785f53c`.
    ```solidity
    function removeAdmin(address admin) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct removeAdminCall {
        #[allow(missing_docs)]
        pub admin: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`removeAdmin(address)`](removeAdminCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct removeAdminReturn {}
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
            impl ::core::convert::From<removeAdminCall> for UnderlyingRustTuple<'_> {
                fn from(value: removeAdminCall) -> Self {
                    (value.admin,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for removeAdminCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { admin: tuple.0 }
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
            impl ::core::convert::From<removeAdminReturn> for UnderlyingRustTuple<'_> {
                fn from(value: removeAdminReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for removeAdminReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for removeAdminCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = removeAdminReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "removeAdmin(address)";
            const SELECTOR: [u8; 4] = [23u8, 133u8, 245u8, 60u8];
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
                        &self.admin,
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
    /**Function with signature `removeAppointee(address,address,bytes4)` and selector `0xba550880`.
    ```solidity
    function removeAppointee(address appointee, address target, bytes4 selector) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct removeAppointeeCall {
        #[allow(missing_docs)]
        pub appointee: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub target: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub selector: alloy::sol_types::private::FixedBytes<4>,
    }
    ///Container type for the return parameters of the [`removeAppointee(address,address,bytes4)`](removeAppointeeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct removeAppointeeReturn {}
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
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::FixedBytes<4>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::FixedBytes<4>,
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
            impl ::core::convert::From<removeAppointeeCall> for UnderlyingRustTuple<'_> {
                fn from(value: removeAppointeeCall) -> Self {
                    (value.appointee, value.target, value.selector)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for removeAppointeeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        appointee: tuple.0,
                        target: tuple.1,
                        selector: tuple.2,
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
            impl ::core::convert::From<removeAppointeeReturn> for UnderlyingRustTuple<'_> {
                fn from(value: removeAppointeeReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for removeAppointeeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for removeAppointeeCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::FixedBytes<4>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = removeAppointeeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "removeAppointee(address,address,bytes4)";
            const SELECTOR: [u8; 4] = [186u8, 85u8, 8u8, 128u8];
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
                        &self.appointee,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.target,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        4,
                    > as alloy_sol_types::SolType>::tokenize(&self.selector),
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
    /**Function with signature `removePendingAdmin(address)` and selector `0x9da16d8e`.
    ```solidity
    function removePendingAdmin(address pendingAdmin) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct removePendingAdminCall {
        #[allow(missing_docs)]
        pub pendingAdmin: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`removePendingAdmin(address)`](removePendingAdminCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct removePendingAdminReturn {}
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
            impl ::core::convert::From<removePendingAdminCall> for UnderlyingRustTuple<'_> {
                fn from(value: removePendingAdminCall) -> Self {
                    (value.pendingAdmin,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for removePendingAdminCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        pendingAdmin: tuple.0,
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
            impl ::core::convert::From<removePendingAdminReturn> for UnderlyingRustTuple<'_> {
                fn from(value: removePendingAdminReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for removePendingAdminReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for removePendingAdminCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = removePendingAdminReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "removePendingAdmin(address)";
            const SELECTOR: [u8; 4] = [157u8, 161u8, 109u8, 142u8];
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
                        &self.pendingAdmin,
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
    /**Function with signature `respondToTask((string,uint32),uint32,bytes)` and selector `0x3415a49c`.
    ```solidity
    function respondToTask(IHelloWorldServiceManager.Task memory task, uint32 referenceTaskIndex, bytes memory signature) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct respondToTaskCall {
        #[allow(missing_docs)]
        pub task: <IHelloWorldServiceManager::Task as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub referenceTaskIndex: u32,
        #[allow(missing_docs)]
        pub signature: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`respondToTask((string,uint32),uint32,bytes)`](respondToTaskCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct respondToTaskReturn {}
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
                IHelloWorldServiceManager::Task,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IHelloWorldServiceManager::Task as alloy::sol_types::SolType>::RustType,
                u32,
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
            impl ::core::convert::From<respondToTaskCall> for UnderlyingRustTuple<'_> {
                fn from(value: respondToTaskCall) -> Self {
                    (value.task, value.referenceTaskIndex, value.signature)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for respondToTaskCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        task: tuple.0,
                        referenceTaskIndex: tuple.1,
                        signature: tuple.2,
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
            impl ::core::convert::From<respondToTaskReturn> for UnderlyingRustTuple<'_> {
                fn from(value: respondToTaskReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for respondToTaskReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for respondToTaskCall {
            type Parameters<'a> = (
                IHelloWorldServiceManager::Task,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = respondToTaskReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "respondToTask((string,uint32),uint32,bytes)";
            const SELECTOR: [u8; 4] = [52u8, 21u8, 164u8, 156u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <IHelloWorldServiceManager::Task as alloy_sol_types::SolType>::tokenize(
                        &self.task,
                    ),
                    <alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::SolType>::tokenize(
                        &self.referenceTaskIndex,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.signature,
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
    /**Function with signature `rewardsInitiator()` and selector `0xfc299dee`.
    ```solidity
    function rewardsInitiator() external view returns (address);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct rewardsInitiatorCall {}
    ///Container type for the return parameters of the [`rewardsInitiator()`](rewardsInitiatorCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct rewardsInitiatorReturn {
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
            impl ::core::convert::From<rewardsInitiatorCall> for UnderlyingRustTuple<'_> {
                fn from(value: rewardsInitiatorCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for rewardsInitiatorCall {
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
            impl ::core::convert::From<rewardsInitiatorReturn> for UnderlyingRustTuple<'_> {
                fn from(value: rewardsInitiatorReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for rewardsInitiatorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for rewardsInitiatorCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = rewardsInitiatorReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "rewardsInitiator()";
            const SELECTOR: [u8; 4] = [252u8, 41u8, 157u8, 238u8];
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
    /**Function with signature `setAVSRegistrar(address)` and selector `0xf25f1610`.
    ```solidity
    function setAVSRegistrar(address registrar) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setAVSRegistrarCall {
        #[allow(missing_docs)]
        pub registrar: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`setAVSRegistrar(address)`](setAVSRegistrarCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setAVSRegistrarReturn {}
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
            impl ::core::convert::From<setAVSRegistrarCall> for UnderlyingRustTuple<'_> {
                fn from(value: setAVSRegistrarCall) -> Self {
                    (value.registrar,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setAVSRegistrarCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { registrar: tuple.0 }
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
            impl ::core::convert::From<setAVSRegistrarReturn> for UnderlyingRustTuple<'_> {
                fn from(value: setAVSRegistrarReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setAVSRegistrarReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setAVSRegistrarCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = setAVSRegistrarReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setAVSRegistrar(address)";
            const SELECTOR: [u8; 4] = [242u8, 95u8, 22u8, 16u8];
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
                        &self.registrar,
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
    /**Function with signature `setAppointee(address,address,bytes4)` and selector `0x1fdb0cfd`.
    ```solidity
    function setAppointee(address appointee, address target, bytes4 selector) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setAppointeeCall {
        #[allow(missing_docs)]
        pub appointee: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub target: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub selector: alloy::sol_types::private::FixedBytes<4>,
    }
    ///Container type for the return parameters of the [`setAppointee(address,address,bytes4)`](setAppointeeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setAppointeeReturn {}
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
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::FixedBytes<4>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::FixedBytes<4>,
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
            impl ::core::convert::From<setAppointeeCall> for UnderlyingRustTuple<'_> {
                fn from(value: setAppointeeCall) -> Self {
                    (value.appointee, value.target, value.selector)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setAppointeeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        appointee: tuple.0,
                        target: tuple.1,
                        selector: tuple.2,
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
            impl ::core::convert::From<setAppointeeReturn> for UnderlyingRustTuple<'_> {
                fn from(value: setAppointeeReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setAppointeeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setAppointeeCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::FixedBytes<4>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = setAppointeeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setAppointee(address,address,bytes4)";
            const SELECTOR: [u8; 4] = [31u8, 219u8, 12u8, 253u8];
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
                        &self.appointee,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.target,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        4,
                    > as alloy_sol_types::SolType>::tokenize(&self.selector),
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
    /**Function with signature `setClaimerFor(address)` and selector `0xa0169ddd`.
    ```solidity
    function setClaimerFor(address claimer) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setClaimerForCall {
        #[allow(missing_docs)]
        pub claimer: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`setClaimerFor(address)`](setClaimerForCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setClaimerForReturn {}
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
            impl ::core::convert::From<setClaimerForCall> for UnderlyingRustTuple<'_> {
                fn from(value: setClaimerForCall) -> Self {
                    (value.claimer,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setClaimerForCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { claimer: tuple.0 }
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
            impl ::core::convert::From<setClaimerForReturn> for UnderlyingRustTuple<'_> {
                fn from(value: setClaimerForReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setClaimerForReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setClaimerForCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = setClaimerForReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setClaimerFor(address)";
            const SELECTOR: [u8; 4] = [160u8, 22u8, 157u8, 221u8];
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
                        &self.claimer,
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
    /**Function with signature `setRewardsInitiator(address)` and selector `0x3bc28c8c`.
    ```solidity
    function setRewardsInitiator(address newRewardsInitiator) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setRewardsInitiatorCall {
        #[allow(missing_docs)]
        pub newRewardsInitiator: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`setRewardsInitiator(address)`](setRewardsInitiatorCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setRewardsInitiatorReturn {}
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
            impl ::core::convert::From<setRewardsInitiatorCall> for UnderlyingRustTuple<'_> {
                fn from(value: setRewardsInitiatorCall) -> Self {
                    (value.newRewardsInitiator,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setRewardsInitiatorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        newRewardsInitiator: tuple.0,
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
            impl ::core::convert::From<setRewardsInitiatorReturn> for UnderlyingRustTuple<'_> {
                fn from(value: setRewardsInitiatorReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setRewardsInitiatorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setRewardsInitiatorCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = setRewardsInitiatorReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setRewardsInitiator(address)";
            const SELECTOR: [u8; 4] = [59u8, 194u8, 140u8, 140u8];
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
                        &self.newRewardsInitiator,
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
    /**Function with signature `slashOperator((string,uint32),uint32,address)` and selector `0x9677de10`.
    ```solidity
    function slashOperator(IHelloWorldServiceManager.Task memory task, uint32 referenceTaskIndex, address operator) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct slashOperatorCall {
        #[allow(missing_docs)]
        pub task: <IHelloWorldServiceManager::Task as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub referenceTaskIndex: u32,
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`slashOperator((string,uint32),uint32,address)`](slashOperatorCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct slashOperatorReturn {}
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
                IHelloWorldServiceManager::Task,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IHelloWorldServiceManager::Task as alloy::sol_types::SolType>::RustType,
                u32,
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
            impl ::core::convert::From<slashOperatorCall> for UnderlyingRustTuple<'_> {
                fn from(value: slashOperatorCall) -> Self {
                    (value.task, value.referenceTaskIndex, value.operator)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for slashOperatorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        task: tuple.0,
                        referenceTaskIndex: tuple.1,
                        operator: tuple.2,
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
            impl ::core::convert::From<slashOperatorReturn> for UnderlyingRustTuple<'_> {
                fn from(value: slashOperatorReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for slashOperatorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for slashOperatorCall {
            type Parameters<'a> = (
                IHelloWorldServiceManager::Task,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = slashOperatorReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "slashOperator((string,uint32),uint32,address)";
            const SELECTOR: [u8; 4] = [150u8, 119u8, 222u8, 16u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <IHelloWorldServiceManager::Task as alloy_sol_types::SolType>::tokenize(
                        &self.task,
                    ),
                    <alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::SolType>::tokenize(
                        &self.referenceTaskIndex,
                    ),
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
    /**Function with signature `stakeRegistry()` and selector `0x68304835`.
    ```solidity
    function stakeRegistry() external view returns (address);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct stakeRegistryCall {}
    ///Container type for the return parameters of the [`stakeRegistry()`](stakeRegistryCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct stakeRegistryReturn {
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
            impl ::core::convert::From<stakeRegistryCall> for UnderlyingRustTuple<'_> {
                fn from(value: stakeRegistryCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for stakeRegistryCall {
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
            impl ::core::convert::From<stakeRegistryReturn> for UnderlyingRustTuple<'_> {
                fn from(value: stakeRegistryReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for stakeRegistryReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for stakeRegistryCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = stakeRegistryReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "stakeRegistry()";
            const SELECTOR: [u8; 4] = [104u8, 48u8, 72u8, 53u8];
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
    /**Function with signature `taskWasResponded(uint32)` and selector `0x7b6c4236`.
    ```solidity
    function taskWasResponded(uint32) external view returns (bool);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct taskWasRespondedCall {
        #[allow(missing_docs)]
        pub _0: u32,
    }
    ///Container type for the return parameters of the [`taskWasResponded(uint32)`](taskWasRespondedCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct taskWasRespondedReturn {
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
            impl ::core::convert::From<taskWasRespondedCall> for UnderlyingRustTuple<'_> {
                fn from(value: taskWasRespondedCall) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for taskWasRespondedCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
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
            impl ::core::convert::From<taskWasRespondedReturn> for UnderlyingRustTuple<'_> {
                fn from(value: taskWasRespondedReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for taskWasRespondedReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for taskWasRespondedCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = taskWasRespondedReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "taskWasResponded(uint32)";
            const SELECTOR: [u8; 4] = [123u8, 108u8, 66u8, 54u8];
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
                        &self._0,
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
    /**Function with signature `updateAVSMetadataURI(string)` and selector `0xa98fb355`.
    ```solidity
    function updateAVSMetadataURI(string memory _metadataURI) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateAVSMetadataURICall {
        #[allow(missing_docs)]
        pub _metadataURI: alloy::sol_types::private::String,
    }
    ///Container type for the return parameters of the [`updateAVSMetadataURI(string)`](updateAVSMetadataURICall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateAVSMetadataURIReturn {}
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
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::String,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::String,);
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
            impl ::core::convert::From<updateAVSMetadataURICall> for UnderlyingRustTuple<'_> {
                fn from(value: updateAVSMetadataURICall) -> Self {
                    (value._metadataURI,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for updateAVSMetadataURICall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _metadataURI: tuple.0,
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
            impl ::core::convert::From<updateAVSMetadataURIReturn> for UnderlyingRustTuple<'_> {
                fn from(value: updateAVSMetadataURIReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for updateAVSMetadataURIReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for updateAVSMetadataURICall {
            type Parameters<'a> = (alloy::sol_types::sol_data::String,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = updateAVSMetadataURIReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "updateAVSMetadataURI(string)";
            const SELECTOR: [u8; 4] = [169u8, 143u8, 179u8, 85u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self._metadataURI,
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
    ///Container for all the [`HelloWorldServiceManager`](self) function calls.
    pub enum HelloWorldServiceManagerCalls {
        #[allow(missing_docs)]
        MAX_RESPONSE_INTERVAL_BLOCKS(MAX_RESPONSE_INTERVAL_BLOCKSCall),
        #[allow(missing_docs)]
        addPendingAdmin(addPendingAdminCall),
        #[allow(missing_docs)]
        allTaskHashes(allTaskHashesCall),
        #[allow(missing_docs)]
        allTaskResponses(allTaskResponsesCall),
        #[allow(missing_docs)]
        allocationManager(allocationManagerCall),
        #[allow(missing_docs)]
        avsDirectory(avsDirectoryCall),
        #[allow(missing_docs)]
        createAVSRewardsSubmission(createAVSRewardsSubmissionCall),
        #[allow(missing_docs)]
        createNewTask(createNewTaskCall),
        #[allow(missing_docs)]
        createOperatorDirectedAVSRewardsSubmission(createOperatorDirectedAVSRewardsSubmissionCall),
        #[allow(missing_docs)]
        deregisterOperatorFromAVS(deregisterOperatorFromAVSCall),
        #[allow(missing_docs)]
        deregisterOperatorFromOperatorSets(deregisterOperatorFromOperatorSetsCall),
        #[allow(missing_docs)]
        getOperatorRestakedStrategies(getOperatorRestakedStrategiesCall),
        #[allow(missing_docs)]
        getRestakeableStrategies(getRestakeableStrategiesCall),
        #[allow(missing_docs)]
        initialize(initializeCall),
        #[allow(missing_docs)]
        latestTaskNum(latestTaskNumCall),
        #[allow(missing_docs)]
        owner(ownerCall),
        #[allow(missing_docs)]
        registerOperatorToAVS(registerOperatorToAVSCall),
        #[allow(missing_docs)]
        removeAdmin(removeAdminCall),
        #[allow(missing_docs)]
        removeAppointee(removeAppointeeCall),
        #[allow(missing_docs)]
        removePendingAdmin(removePendingAdminCall),
        #[allow(missing_docs)]
        renounceOwnership(renounceOwnershipCall),
        #[allow(missing_docs)]
        respondToTask(respondToTaskCall),
        #[allow(missing_docs)]
        rewardsInitiator(rewardsInitiatorCall),
        #[allow(missing_docs)]
        setAVSRegistrar(setAVSRegistrarCall),
        #[allow(missing_docs)]
        setAppointee(setAppointeeCall),
        #[allow(missing_docs)]
        setClaimerFor(setClaimerForCall),
        #[allow(missing_docs)]
        setRewardsInitiator(setRewardsInitiatorCall),
        #[allow(missing_docs)]
        slashOperator(slashOperatorCall),
        #[allow(missing_docs)]
        stakeRegistry(stakeRegistryCall),
        #[allow(missing_docs)]
        taskWasResponded(taskWasRespondedCall),
        #[allow(missing_docs)]
        transferOwnership(transferOwnershipCall),
        #[allow(missing_docs)]
        updateAVSMetadataURI(updateAVSMetadataURICall),
    }
    #[automatically_derived]
    impl HelloWorldServiceManagerCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [23u8, 133u8, 245u8, 60u8],
            [31u8, 219u8, 12u8, 253u8],
            [39u8, 148u8, 50u8, 235u8],
            [45u8, 137u8, 246u8, 252u8],
            [51u8, 207u8, 183u8, 183u8],
            [52u8, 21u8, 164u8, 156u8],
            [59u8, 194u8, 140u8, 140u8],
            [72u8, 92u8, 201u8, 85u8],
            [104u8, 48u8, 72u8, 53u8],
            [107u8, 58u8, 167u8, 46u8],
            [113u8, 80u8, 24u8, 166u8],
            [115u8, 61u8, 5u8, 108u8],
            [123u8, 108u8, 66u8, 54u8],
            [133u8, 237u8, 248u8, 116u8],
            [139u8, 0u8, 206u8, 124u8],
            [141u8, 165u8, 203u8, 91u8],
            [150u8, 119u8, 222u8, 16u8],
            [153u8, 38u8, 238u8, 125u8],
            [157u8, 161u8, 109u8, 142u8],
            [160u8, 22u8, 157u8, 221u8],
            [162u8, 11u8, 153u8, 191u8],
            [163u8, 100u8, 244u8, 218u8],
            [169u8, 143u8, 179u8, 85u8],
            [186u8, 85u8, 8u8, 128u8],
            [193u8, 168u8, 226u8, 197u8],
            [194u8, 11u8, 171u8, 127u8],
            [202u8, 138u8, 167u8, 199u8],
            [228u8, 129u8, 175u8, 157u8],
            [242u8, 95u8, 22u8, 16u8],
            [242u8, 253u8, 227u8, 139u8],
            [252u8, 41u8, 157u8, 238u8],
            [252u8, 227u8, 108u8, 125u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for HelloWorldServiceManagerCalls {
        const NAME: &'static str = "HelloWorldServiceManagerCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 32usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::MAX_RESPONSE_INTERVAL_BLOCKS(_) => {
                    <MAX_RESPONSE_INTERVAL_BLOCKSCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::addPendingAdmin(_) => {
                    <addPendingAdminCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::allTaskHashes(_) => {
                    <allTaskHashesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::allTaskResponses(_) => {
                    <allTaskResponsesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::allocationManager(_) => {
                    <allocationManagerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::avsDirectory(_) => {
                    <avsDirectoryCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::createAVSRewardsSubmission(_) => {
                    <createAVSRewardsSubmissionCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::createNewTask(_) => {
                    <createNewTaskCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::createOperatorDirectedAVSRewardsSubmission(_) => {
                    <createOperatorDirectedAVSRewardsSubmissionCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::deregisterOperatorFromAVS(_) => {
                    <deregisterOperatorFromAVSCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::deregisterOperatorFromOperatorSets(_) => {
                    <deregisterOperatorFromOperatorSetsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getOperatorRestakedStrategies(_) => {
                    <getOperatorRestakedStrategiesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getRestakeableStrategies(_) => {
                    <getRestakeableStrategiesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::initialize(_) => {
                    <initializeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::latestTaskNum(_) => {
                    <latestTaskNumCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::owner(_) => <ownerCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::registerOperatorToAVS(_) => {
                    <registerOperatorToAVSCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::removeAdmin(_) => {
                    <removeAdminCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::removeAppointee(_) => {
                    <removeAppointeeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::removePendingAdmin(_) => {
                    <removePendingAdminCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::renounceOwnership(_) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::respondToTask(_) => {
                    <respondToTaskCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::rewardsInitiator(_) => {
                    <rewardsInitiatorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setAVSRegistrar(_) => {
                    <setAVSRegistrarCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setAppointee(_) => {
                    <setAppointeeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setClaimerFor(_) => {
                    <setClaimerForCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setRewardsInitiator(_) => {
                    <setRewardsInitiatorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::slashOperator(_) => {
                    <slashOperatorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::stakeRegistry(_) => {
                    <stakeRegistryCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::taskWasResponded(_) => {
                    <taskWasRespondedCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::transferOwnership(_) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::updateAVSMetadataURI(_) => {
                    <updateAVSMetadataURICall as alloy_sol_types::SolCall>::SELECTOR
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
                -> alloy_sol_types::Result<HelloWorldServiceManagerCalls>] = &[
                {
                    fn removeAdmin(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<HelloWorldServiceManagerCalls>
                    {
                        <removeAdminCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(HelloWorldServiceManagerCalls::removeAdmin)
                    }
                    removeAdmin
                },
                {
                    fn setAppointee(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<HelloWorldServiceManagerCalls>
                    {
                        <setAppointeeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(HelloWorldServiceManagerCalls::setAppointee)
                    }
                    setAppointee
                },
                {
                    fn addPendingAdmin(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<HelloWorldServiceManagerCalls>
                    {
                        <addPendingAdminCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(HelloWorldServiceManagerCalls::addPendingAdmin)
                    }
                    addPendingAdmin
                },
                {
                    fn allTaskHashes(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<HelloWorldServiceManagerCalls>
                    {
                        <allTaskHashesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(HelloWorldServiceManagerCalls::allTaskHashes)
                    }
                    allTaskHashes
                },
                {
                    fn getOperatorRestakedStrategies(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<HelloWorldServiceManagerCalls>
                    {
                        <getOperatorRestakedStrategiesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                HelloWorldServiceManagerCalls::getOperatorRestakedStrategies,
                            )
                    }
                    getOperatorRestakedStrategies
                },
                {
                    fn respondToTask(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<HelloWorldServiceManagerCalls>
                    {
                        <respondToTaskCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(HelloWorldServiceManagerCalls::respondToTask)
                    }
                    respondToTask
                },
                {
                    fn setRewardsInitiator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<HelloWorldServiceManagerCalls>
                    {
                        <setRewardsInitiatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(HelloWorldServiceManagerCalls::setRewardsInitiator)
                    }
                    setRewardsInitiator
                },
                {
                    fn initialize(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<HelloWorldServiceManagerCalls>
                    {
                        <initializeCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(HelloWorldServiceManagerCalls::initialize)
                    }
                    initialize
                },
                {
                    fn stakeRegistry(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<HelloWorldServiceManagerCalls>
                    {
                        <stakeRegistryCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(HelloWorldServiceManagerCalls::stakeRegistry)
                    }
                    stakeRegistry
                },
                {
                    fn avsDirectory(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<HelloWorldServiceManagerCalls>
                    {
                        <avsDirectoryCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(HelloWorldServiceManagerCalls::avsDirectory)
                    }
                    avsDirectory
                },
                {
                    fn renounceOwnership(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<HelloWorldServiceManagerCalls>
                    {
                        <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(HelloWorldServiceManagerCalls::renounceOwnership)
                    }
                    renounceOwnership
                },
                {
                    fn MAX_RESPONSE_INTERVAL_BLOCKS(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<HelloWorldServiceManagerCalls>
                    {
                        <MAX_RESPONSE_INTERVAL_BLOCKSCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                HelloWorldServiceManagerCalls::MAX_RESPONSE_INTERVAL_BLOCKS,
                            )
                    }
                    MAX_RESPONSE_INTERVAL_BLOCKS
                },
                {
                    fn taskWasResponded(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<HelloWorldServiceManagerCalls>
                    {
                        <taskWasRespondedCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(HelloWorldServiceManagerCalls::taskWasResponded)
                    }
                    taskWasResponded
                },
                {
                    fn createNewTask(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<HelloWorldServiceManagerCalls>
                    {
                        <createNewTaskCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(HelloWorldServiceManagerCalls::createNewTask)
                    }
                    createNewTask
                },
                {
                    fn latestTaskNum(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<HelloWorldServiceManagerCalls>
                    {
                        <latestTaskNumCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(HelloWorldServiceManagerCalls::latestTaskNum)
                    }
                    latestTaskNum
                },
                {
                    fn owner(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<HelloWorldServiceManagerCalls>
                    {
                        <ownerCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(HelloWorldServiceManagerCalls::owner)
                    }
                    owner
                },
                {
                    fn slashOperator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<HelloWorldServiceManagerCalls>
                    {
                        <slashOperatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(HelloWorldServiceManagerCalls::slashOperator)
                    }
                    slashOperator
                },
                {
                    fn registerOperatorToAVS(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<HelloWorldServiceManagerCalls>
                    {
                        <registerOperatorToAVSCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(HelloWorldServiceManagerCalls::registerOperatorToAVS)
                    }
                    registerOperatorToAVS
                },
                {
                    fn removePendingAdmin(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<HelloWorldServiceManagerCalls>
                    {
                        <removePendingAdminCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(HelloWorldServiceManagerCalls::removePendingAdmin)
                    }
                    removePendingAdmin
                },
                {
                    fn setClaimerFor(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<HelloWorldServiceManagerCalls>
                    {
                        <setClaimerForCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(HelloWorldServiceManagerCalls::setClaimerFor)
                    }
                    setClaimerFor
                },
                {
                    fn createOperatorDirectedAVSRewardsSubmission(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<HelloWorldServiceManagerCalls>
                    {
                        <createOperatorDirectedAVSRewardsSubmissionCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                HelloWorldServiceManagerCalls::createOperatorDirectedAVSRewardsSubmission,
                            )
                    }
                    createOperatorDirectedAVSRewardsSubmission
                },
                {
                    fn deregisterOperatorFromAVS(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<HelloWorldServiceManagerCalls>
                    {
                        <deregisterOperatorFromAVSCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(HelloWorldServiceManagerCalls::deregisterOperatorFromAVS)
                    }
                    deregisterOperatorFromAVS
                },
                {
                    fn updateAVSMetadataURI(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<HelloWorldServiceManagerCalls>
                    {
                        <updateAVSMetadataURICall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(HelloWorldServiceManagerCalls::updateAVSMetadataURI)
                    }
                    updateAVSMetadataURI
                },
                {
                    fn removeAppointee(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<HelloWorldServiceManagerCalls>
                    {
                        <removeAppointeeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(HelloWorldServiceManagerCalls::removeAppointee)
                    }
                    removeAppointee
                },
                {
                    fn deregisterOperatorFromOperatorSets(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<HelloWorldServiceManagerCalls>
                    {
                        <deregisterOperatorFromOperatorSetsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                HelloWorldServiceManagerCalls::deregisterOperatorFromOperatorSets,
                            )
                    }
                    deregisterOperatorFromOperatorSets
                },
                {
                    fn allTaskResponses(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<HelloWorldServiceManagerCalls>
                    {
                        <allTaskResponsesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(HelloWorldServiceManagerCalls::allTaskResponses)
                    }
                    allTaskResponses
                },
                {
                    fn allocationManager(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<HelloWorldServiceManagerCalls>
                    {
                        <allocationManagerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(HelloWorldServiceManagerCalls::allocationManager)
                    }
                    allocationManager
                },
                {
                    fn getRestakeableStrategies(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<HelloWorldServiceManagerCalls>
                    {
                        <getRestakeableStrategiesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(HelloWorldServiceManagerCalls::getRestakeableStrategies)
                    }
                    getRestakeableStrategies
                },
                {
                    fn setAVSRegistrar(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<HelloWorldServiceManagerCalls>
                    {
                        <setAVSRegistrarCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(HelloWorldServiceManagerCalls::setAVSRegistrar)
                    }
                    setAVSRegistrar
                },
                {
                    fn transferOwnership(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<HelloWorldServiceManagerCalls>
                    {
                        <transferOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(HelloWorldServiceManagerCalls::transferOwnership)
                    }
                    transferOwnership
                },
                {
                    fn rewardsInitiator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<HelloWorldServiceManagerCalls>
                    {
                        <rewardsInitiatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(HelloWorldServiceManagerCalls::rewardsInitiator)
                    }
                    rewardsInitiator
                },
                {
                    fn createAVSRewardsSubmission(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<HelloWorldServiceManagerCalls>
                    {
                        <createAVSRewardsSubmissionCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                HelloWorldServiceManagerCalls::createAVSRewardsSubmission,
                            )
                    }
                    createAVSRewardsSubmission
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
                Self::MAX_RESPONSE_INTERVAL_BLOCKS(inner) => {
                    <MAX_RESPONSE_INTERVAL_BLOCKSCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::addPendingAdmin(inner) => {
                    <addPendingAdminCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::allTaskHashes(inner) => {
                    <allTaskHashesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::allTaskResponses(inner) => {
                    <allTaskResponsesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::allocationManager(inner) => {
                    <allocationManagerCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::avsDirectory(inner) => {
                    <avsDirectoryCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::createAVSRewardsSubmission(inner) => {
                    <createAVSRewardsSubmissionCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::createNewTask(inner) => {
                    <createNewTaskCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::createOperatorDirectedAVSRewardsSubmission(inner) => {
                    <createOperatorDirectedAVSRewardsSubmissionCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::deregisterOperatorFromAVS(inner) => {
                    <deregisterOperatorFromAVSCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::deregisterOperatorFromOperatorSets(inner) => {
                    <deregisterOperatorFromOperatorSetsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getOperatorRestakedStrategies(inner) => {
                    <getOperatorRestakedStrategiesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getRestakeableStrategies(inner) => {
                    <getRestakeableStrategiesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::initialize(inner) => {
                    <initializeCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::latestTaskNum(inner) => {
                    <latestTaskNumCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::owner(inner) => {
                    <ownerCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::registerOperatorToAVS(inner) => {
                    <registerOperatorToAVSCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::removeAdmin(inner) => {
                    <removeAdminCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::removeAppointee(inner) => {
                    <removeAppointeeCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::removePendingAdmin(inner) => {
                    <removePendingAdminCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::renounceOwnership(inner) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::respondToTask(inner) => {
                    <respondToTaskCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::rewardsInitiator(inner) => {
                    <rewardsInitiatorCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setAVSRegistrar(inner) => {
                    <setAVSRegistrarCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setAppointee(inner) => {
                    <setAppointeeCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setClaimerFor(inner) => {
                    <setClaimerForCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setRewardsInitiator(inner) => {
                    <setRewardsInitiatorCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::slashOperator(inner) => {
                    <slashOperatorCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::stakeRegistry(inner) => {
                    <stakeRegistryCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::taskWasResponded(inner) => {
                    <taskWasRespondedCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::transferOwnership(inner) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::updateAVSMetadataURI(inner) => {
                    <updateAVSMetadataURICall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::MAX_RESPONSE_INTERVAL_BLOCKS(inner) => {
                    <MAX_RESPONSE_INTERVAL_BLOCKSCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::addPendingAdmin(inner) => {
                    <addPendingAdminCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::allTaskHashes(inner) => {
                    <allTaskHashesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::allTaskResponses(inner) => {
                    <allTaskResponsesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::allocationManager(inner) => {
                    <allocationManagerCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::avsDirectory(inner) => {
                    <avsDirectoryCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::createAVSRewardsSubmission(inner) => {
                    <createAVSRewardsSubmissionCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::createNewTask(inner) => {
                    <createNewTaskCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::createOperatorDirectedAVSRewardsSubmission(inner) => {
                    <createOperatorDirectedAVSRewardsSubmissionCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::deregisterOperatorFromAVS(inner) => {
                    <deregisterOperatorFromAVSCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::deregisterOperatorFromOperatorSets(inner) => {
                    <deregisterOperatorFromOperatorSetsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getOperatorRestakedStrategies(inner) => {
                    <getOperatorRestakedStrategiesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getRestakeableStrategies(inner) => {
                    <getRestakeableStrategiesCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::latestTaskNum(inner) => {
                    <latestTaskNumCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::owner(inner) => {
                    <ownerCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::registerOperatorToAVS(inner) => {
                    <registerOperatorToAVSCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::removeAdmin(inner) => {
                    <removeAdminCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::removeAppointee(inner) => {
                    <removeAppointeeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::removePendingAdmin(inner) => {
                    <removePendingAdminCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::respondToTask(inner) => {
                    <respondToTaskCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::rewardsInitiator(inner) => {
                    <rewardsInitiatorCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setAVSRegistrar(inner) => {
                    <setAVSRegistrarCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setAppointee(inner) => {
                    <setAppointeeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setClaimerFor(inner) => {
                    <setClaimerForCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setRewardsInitiator(inner) => {
                    <setRewardsInitiatorCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::slashOperator(inner) => {
                    <slashOperatorCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::stakeRegistry(inner) => {
                    <stakeRegistryCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::taskWasResponded(inner) => {
                    <taskWasRespondedCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::updateAVSMetadataURI(inner) => {
                    <updateAVSMetadataURICall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`HelloWorldServiceManager`](self) custom errors.
    pub enum HelloWorldServiceManagerErrors {
        #[allow(missing_docs)]
        DelayPeriodNotPassed(DelayPeriodNotPassed),
        #[allow(missing_docs)]
        OnlyRegistryCoordinator(OnlyRegistryCoordinator),
        #[allow(missing_docs)]
        OnlyRewardsInitiator(OnlyRewardsInitiator),
        #[allow(missing_docs)]
        OnlyStakeRegistry(OnlyStakeRegistry),
    }
    #[automatically_derived]
    impl HelloWorldServiceManagerErrors {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [70u8, 191u8, 34u8, 129u8],
            [135u8, 41u8, 183u8, 190u8],
            [142u8, 121u8, 253u8, 181u8],
            [251u8, 98u8, 59u8, 4u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for HelloWorldServiceManagerErrors {
        const NAME: &'static str = "HelloWorldServiceManagerErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 4usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::DelayPeriodNotPassed(_) => {
                    <DelayPeriodNotPassed as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OnlyRegistryCoordinator(_) => {
                    <OnlyRegistryCoordinator as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OnlyRewardsInitiator(_) => {
                    <OnlyRewardsInitiator as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OnlyStakeRegistry(_) => {
                    <OnlyStakeRegistry as alloy_sol_types::SolError>::SELECTOR
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
            ) -> alloy_sol_types::Result<
                HelloWorldServiceManagerErrors,
            >] = &[
                {
                    fn OnlyStakeRegistry(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<HelloWorldServiceManagerErrors>
                    {
                        <OnlyStakeRegistry as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(HelloWorldServiceManagerErrors::OnlyStakeRegistry)
                    }
                    OnlyStakeRegistry
                },
                {
                    fn OnlyRegistryCoordinator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<HelloWorldServiceManagerErrors>
                    {
                        <OnlyRegistryCoordinator as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(HelloWorldServiceManagerErrors::OnlyRegistryCoordinator)
                    }
                    OnlyRegistryCoordinator
                },
                {
                    fn OnlyRewardsInitiator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<HelloWorldServiceManagerErrors>
                    {
                        <OnlyRewardsInitiator as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(HelloWorldServiceManagerErrors::OnlyRewardsInitiator)
                    }
                    OnlyRewardsInitiator
                },
                {
                    fn DelayPeriodNotPassed(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<HelloWorldServiceManagerErrors>
                    {
                        <DelayPeriodNotPassed as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(HelloWorldServiceManagerErrors::DelayPeriodNotPassed)
                    }
                    DelayPeriodNotPassed
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
                Self::DelayPeriodNotPassed(inner) => {
                    <DelayPeriodNotPassed as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::OnlyRegistryCoordinator(inner) => {
                    <OnlyRegistryCoordinator as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::OnlyRewardsInitiator(inner) => {
                    <OnlyRewardsInitiator as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::OnlyStakeRegistry(inner) => {
                    <OnlyStakeRegistry as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::DelayPeriodNotPassed(inner) => {
                    <DelayPeriodNotPassed as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::OnlyRegistryCoordinator(inner) => {
                    <OnlyRegistryCoordinator as alloy_sol_types::SolError>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::OnlyRewardsInitiator(inner) => {
                    <OnlyRewardsInitiator as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::OnlyStakeRegistry(inner) => {
                    <OnlyStakeRegistry as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
            }
        }
    }
    ///Container for all the [`HelloWorldServiceManager`](self) events.
    pub enum HelloWorldServiceManagerEvents {
        #[allow(missing_docs)]
        Initialized(Initialized),
        #[allow(missing_docs)]
        NewTaskCreated(NewTaskCreated),
        #[allow(missing_docs)]
        OwnershipTransferred(OwnershipTransferred),
        #[allow(missing_docs)]
        RewardsInitiatorUpdated(RewardsInitiatorUpdated),
        #[allow(missing_docs)]
        TaskResponded(TaskResponded),
    }
    #[automatically_derived]
    impl HelloWorldServiceManagerEvents {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 32usize]] = &[
            [
                88u8, 24u8, 10u8, 106u8, 4u8, 3u8, 166u8, 60u8, 43u8, 92u8, 228u8, 184u8, 93u8,
                18u8, 157u8, 70u8, 168u8, 13u8, 55u8, 133u8, 27u8, 34u8, 22u8, 189u8, 10u8, 152u8,
                181u8, 158u8, 115u8, 9u8, 184u8, 71u8,
            ],
            [
                127u8, 38u8, 184u8, 63u8, 249u8, 110u8, 31u8, 43u8, 106u8, 104u8, 47u8, 19u8, 56u8,
                82u8, 246u8, 121u8, 138u8, 9u8, 196u8, 101u8, 218u8, 149u8, 146u8, 20u8, 96u8,
                206u8, 251u8, 56u8, 71u8, 64u8, 36u8, 152u8,
            ],
            [
                139u8, 224u8, 7u8, 156u8, 83u8, 22u8, 89u8, 20u8, 19u8, 68u8, 205u8, 31u8, 208u8,
                164u8, 242u8, 132u8, 25u8, 73u8, 127u8, 151u8, 34u8, 163u8, 218u8, 175u8, 227u8,
                180u8, 24u8, 111u8, 107u8, 100u8, 87u8, 224u8,
            ],
            [
                142u8, 178u8, 210u8, 252u8, 204u8, 245u8, 128u8, 30u8, 16u8, 255u8, 88u8, 205u8,
                115u8, 232u8, 120u8, 27u8, 169u8, 35u8, 18u8, 41u8, 99u8, 120u8, 147u8, 120u8,
                119u8, 31u8, 3u8, 193u8, 20u8, 139u8, 2u8, 62u8,
            ],
            [
                225u8, 28u8, 221u8, 241u8, 129u8, 106u8, 67u8, 49u8, 140u8, 161u8, 117u8, 187u8,
                197u8, 44u8, 208u8, 24u8, 84u8, 54u8, 233u8, 203u8, 234u8, 215u8, 200u8, 58u8,
                204u8, 84u8, 167u8, 62u8, 70u8, 23u8, 23u8, 227u8,
            ],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolEventInterface for HelloWorldServiceManagerEvents {
        const NAME: &'static str = "HelloWorldServiceManagerEvents";
        const COUNT: usize = 5usize;
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
                Some(<NewTaskCreated as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <NewTaskCreated as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::NewTaskCreated)
                }
                Some(<OwnershipTransferred as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <OwnershipTransferred as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::OwnershipTransferred)
                }
                Some(<RewardsInitiatorUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <RewardsInitiatorUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::RewardsInitiatorUpdated)
                }
                Some(<TaskResponded as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <TaskResponded as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::TaskResponded)
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
    impl alloy_sol_types::private::IntoLogData for HelloWorldServiceManagerEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::Initialized(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::NewTaskCreated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OwnershipTransferred(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::RewardsInitiatorUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::TaskResponded(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
            }
        }
        fn into_log_data(self) -> alloy_sol_types::private::LogData {
            match self {
                Self::Initialized(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::NewTaskCreated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OwnershipTransferred(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::RewardsInitiatorUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::TaskResponded(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`HelloWorldServiceManager`](self) contract instance.

    See the [wrapper's documentation](`HelloWorldServiceManagerInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> HelloWorldServiceManagerInstance<T, P, N> {
        HelloWorldServiceManagerInstance::<T, P, N>::new(address, provider)
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
        _avsDirectory: alloy::sol_types::private::Address,
        _stakeRegistry: alloy::sol_types::private::Address,
        _rewardsCoordinator: alloy::sol_types::private::Address,
        _delegationManager: alloy::sol_types::private::Address,
        _allocationManager: alloy::sol_types::private::Address,
        _maxResponseIntervalBlocks: u32,
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<HelloWorldServiceManagerInstance<T, P, N>>,
    > {
        HelloWorldServiceManagerInstance::<T, P, N>::deploy(
            provider,
            _avsDirectory,
            _stakeRegistry,
            _rewardsCoordinator,
            _delegationManager,
            _allocationManager,
            _maxResponseIntervalBlocks,
        )
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
        _avsDirectory: alloy::sol_types::private::Address,
        _stakeRegistry: alloy::sol_types::private::Address,
        _rewardsCoordinator: alloy::sol_types::private::Address,
        _delegationManager: alloy::sol_types::private::Address,
        _allocationManager: alloy::sol_types::private::Address,
        _maxResponseIntervalBlocks: u32,
    ) -> alloy_contract::RawCallBuilder<T, P, N> {
        HelloWorldServiceManagerInstance::<T, P, N>::deploy_builder(
            provider,
            _avsDirectory,
            _stakeRegistry,
            _rewardsCoordinator,
            _delegationManager,
            _allocationManager,
            _maxResponseIntervalBlocks,
        )
    }
    /**A [`HelloWorldServiceManager`](self) instance.

    Contains type-safe methods for interacting with an on-chain instance of the
    [`HelloWorldServiceManager`](self) contract located at a given `address`, using a given
    provider `P`.

    If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
    documentation on how to provide it), the `deploy` and `deploy_builder` methods can
    be used to deploy a new instance of the contract.

    See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct HelloWorldServiceManagerInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for HelloWorldServiceManagerInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("HelloWorldServiceManagerInstance")
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
        > HelloWorldServiceManagerInstance<T, P, N>
    {
        /**Creates a new wrapper around an on-chain [`HelloWorldServiceManager`](self) contract instance.

        See the [wrapper's documentation](`HelloWorldServiceManagerInstance`) for more details.*/
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
            _avsDirectory: alloy::sol_types::private::Address,
            _stakeRegistry: alloy::sol_types::private::Address,
            _rewardsCoordinator: alloy::sol_types::private::Address,
            _delegationManager: alloy::sol_types::private::Address,
            _allocationManager: alloy::sol_types::private::Address,
            _maxResponseIntervalBlocks: u32,
        ) -> alloy_contract::Result<HelloWorldServiceManagerInstance<T, P, N>> {
            let call_builder = Self::deploy_builder(
                provider,
                _avsDirectory,
                _stakeRegistry,
                _rewardsCoordinator,
                _delegationManager,
                _allocationManager,
                _maxResponseIntervalBlocks,
            );
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
            _avsDirectory: alloy::sol_types::private::Address,
            _stakeRegistry: alloy::sol_types::private::Address,
            _rewardsCoordinator: alloy::sol_types::private::Address,
            _delegationManager: alloy::sol_types::private::Address,
            _allocationManager: alloy::sol_types::private::Address,
            _maxResponseIntervalBlocks: u32,
        ) -> alloy_contract::RawCallBuilder<T, P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                provider,
                [
                    &BYTECODE[..],
                    &alloy_sol_types::SolConstructor::abi_encode(&constructorCall {
                        _avsDirectory,
                        _stakeRegistry,
                        _rewardsCoordinator,
                        _delegationManager,
                        _allocationManager,
                        _maxResponseIntervalBlocks,
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
    impl<T, P: ::core::clone::Clone, N> HelloWorldServiceManagerInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> HelloWorldServiceManagerInstance<T, P, N> {
            HelloWorldServiceManagerInstance {
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
        > HelloWorldServiceManagerInstance<T, P, N>
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
        ///Creates a new call builder for the [`MAX_RESPONSE_INTERVAL_BLOCKS`] function.
        pub fn MAX_RESPONSE_INTERVAL_BLOCKS(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, MAX_RESPONSE_INTERVAL_BLOCKSCall, N> {
            self.call_builder(&MAX_RESPONSE_INTERVAL_BLOCKSCall {})
        }
        ///Creates a new call builder for the [`addPendingAdmin`] function.
        pub fn addPendingAdmin(
            &self,
            admin: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, addPendingAdminCall, N> {
            self.call_builder(&addPendingAdminCall { admin })
        }
        ///Creates a new call builder for the [`allTaskHashes`] function.
        pub fn allTaskHashes(
            &self,
            _0: u32,
        ) -> alloy_contract::SolCallBuilder<T, &P, allTaskHashesCall, N> {
            self.call_builder(&allTaskHashesCall { _0 })
        }
        ///Creates a new call builder for the [`allTaskResponses`] function.
        pub fn allTaskResponses(
            &self,
            _0: alloy::sol_types::private::Address,
            _1: u32,
        ) -> alloy_contract::SolCallBuilder<T, &P, allTaskResponsesCall, N> {
            self.call_builder(&allTaskResponsesCall { _0, _1 })
        }
        ///Creates a new call builder for the [`allocationManager`] function.
        pub fn allocationManager(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, allocationManagerCall, N> {
            self.call_builder(&allocationManagerCall {})
        }
        ///Creates a new call builder for the [`avsDirectory`] function.
        pub fn avsDirectory(&self) -> alloy_contract::SolCallBuilder<T, &P, avsDirectoryCall, N> {
            self.call_builder(&avsDirectoryCall {})
        }
        ///Creates a new call builder for the [`createAVSRewardsSubmission`] function.
        pub fn createAVSRewardsSubmission(
            &self,
            rewardsSubmissions: alloy::sol_types::private::Vec<
                <IRewardsCoordinatorTypes::RewardsSubmission as alloy::sol_types::SolType>::RustType,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, createAVSRewardsSubmissionCall, N> {
            self.call_builder(&createAVSRewardsSubmissionCall { rewardsSubmissions })
        }
        ///Creates a new call builder for the [`createNewTask`] function.
        pub fn createNewTask(
            &self,
            name: alloy::sol_types::private::String,
        ) -> alloy_contract::SolCallBuilder<T, &P, createNewTaskCall, N> {
            self.call_builder(&createNewTaskCall { name })
        }
        ///Creates a new call builder for the [`createOperatorDirectedAVSRewardsSubmission`] function.
        pub fn createOperatorDirectedAVSRewardsSubmission(
            &self,
            operatorDirectedRewardsSubmissions: alloy::sol_types::private::Vec<
                <IRewardsCoordinatorTypes::OperatorDirectedRewardsSubmission as alloy::sol_types::SolType>::RustType,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, createOperatorDirectedAVSRewardsSubmissionCall, N>
        {
            self.call_builder(&createOperatorDirectedAVSRewardsSubmissionCall {
                operatorDirectedRewardsSubmissions,
            })
        }
        ///Creates a new call builder for the [`deregisterOperatorFromAVS`] function.
        pub fn deregisterOperatorFromAVS(
            &self,
            operator: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, deregisterOperatorFromAVSCall, N> {
            self.call_builder(&deregisterOperatorFromAVSCall { operator })
        }
        ///Creates a new call builder for the [`deregisterOperatorFromOperatorSets`] function.
        pub fn deregisterOperatorFromOperatorSets(
            &self,
            operator: alloy::sol_types::private::Address,
            operatorSetIds: alloy::sol_types::private::Vec<u32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, deregisterOperatorFromOperatorSetsCall, N>
        {
            self.call_builder(&deregisterOperatorFromOperatorSetsCall {
                operator,
                operatorSetIds,
            })
        }
        ///Creates a new call builder for the [`getOperatorRestakedStrategies`] function.
        pub fn getOperatorRestakedStrategies(
            &self,
            _operator: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, getOperatorRestakedStrategiesCall, N> {
            self.call_builder(&getOperatorRestakedStrategiesCall { _operator })
        }
        ///Creates a new call builder for the [`getRestakeableStrategies`] function.
        pub fn getRestakeableStrategies(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getRestakeableStrategiesCall, N> {
            self.call_builder(&getRestakeableStrategiesCall {})
        }
        ///Creates a new call builder for the [`initialize`] function.
        pub fn initialize(
            &self,
            initialOwner: alloy::sol_types::private::Address,
            _rewardsInitiator: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, initializeCall, N> {
            self.call_builder(&initializeCall {
                initialOwner,
                _rewardsInitiator,
            })
        }
        ///Creates a new call builder for the [`latestTaskNum`] function.
        pub fn latestTaskNum(&self) -> alloy_contract::SolCallBuilder<T, &P, latestTaskNumCall, N> {
            self.call_builder(&latestTaskNumCall {})
        }
        ///Creates a new call builder for the [`owner`] function.
        pub fn owner(&self) -> alloy_contract::SolCallBuilder<T, &P, ownerCall, N> {
            self.call_builder(&ownerCall {})
        }
        ///Creates a new call builder for the [`registerOperatorToAVS`] function.
        pub fn registerOperatorToAVS(
            &self,
            operator: alloy::sol_types::private::Address,
            operatorSignature: <ISignatureUtilsMixinTypes::SignatureWithSaltAndExpiry as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, registerOperatorToAVSCall, N> {
            self.call_builder(&registerOperatorToAVSCall {
                operator,
                operatorSignature,
            })
        }
        ///Creates a new call builder for the [`removeAdmin`] function.
        pub fn removeAdmin(
            &self,
            admin: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, removeAdminCall, N> {
            self.call_builder(&removeAdminCall { admin })
        }
        ///Creates a new call builder for the [`removeAppointee`] function.
        pub fn removeAppointee(
            &self,
            appointee: alloy::sol_types::private::Address,
            target: alloy::sol_types::private::Address,
            selector: alloy::sol_types::private::FixedBytes<4>,
        ) -> alloy_contract::SolCallBuilder<T, &P, removeAppointeeCall, N> {
            self.call_builder(&removeAppointeeCall {
                appointee,
                target,
                selector,
            })
        }
        ///Creates a new call builder for the [`removePendingAdmin`] function.
        pub fn removePendingAdmin(
            &self,
            pendingAdmin: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, removePendingAdminCall, N> {
            self.call_builder(&removePendingAdminCall { pendingAdmin })
        }
        ///Creates a new call builder for the [`renounceOwnership`] function.
        pub fn renounceOwnership(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, renounceOwnershipCall, N> {
            self.call_builder(&renounceOwnershipCall {})
        }
        ///Creates a new call builder for the [`respondToTask`] function.
        pub fn respondToTask(
            &self,
            task: <IHelloWorldServiceManager::Task as alloy::sol_types::SolType>::RustType,
            referenceTaskIndex: u32,
            signature: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, respondToTaskCall, N> {
            self.call_builder(&respondToTaskCall {
                task,
                referenceTaskIndex,
                signature,
            })
        }
        ///Creates a new call builder for the [`rewardsInitiator`] function.
        pub fn rewardsInitiator(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, rewardsInitiatorCall, N> {
            self.call_builder(&rewardsInitiatorCall {})
        }
        ///Creates a new call builder for the [`setAVSRegistrar`] function.
        pub fn setAVSRegistrar(
            &self,
            registrar: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, setAVSRegistrarCall, N> {
            self.call_builder(&setAVSRegistrarCall { registrar })
        }
        ///Creates a new call builder for the [`setAppointee`] function.
        pub fn setAppointee(
            &self,
            appointee: alloy::sol_types::private::Address,
            target: alloy::sol_types::private::Address,
            selector: alloy::sol_types::private::FixedBytes<4>,
        ) -> alloy_contract::SolCallBuilder<T, &P, setAppointeeCall, N> {
            self.call_builder(&setAppointeeCall {
                appointee,
                target,
                selector,
            })
        }
        ///Creates a new call builder for the [`setClaimerFor`] function.
        pub fn setClaimerFor(
            &self,
            claimer: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, setClaimerForCall, N> {
            self.call_builder(&setClaimerForCall { claimer })
        }
        ///Creates a new call builder for the [`setRewardsInitiator`] function.
        pub fn setRewardsInitiator(
            &self,
            newRewardsInitiator: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, setRewardsInitiatorCall, N> {
            self.call_builder(&setRewardsInitiatorCall {
                newRewardsInitiator,
            })
        }
        ///Creates a new call builder for the [`slashOperator`] function.
        pub fn slashOperator(
            &self,
            task: <IHelloWorldServiceManager::Task as alloy::sol_types::SolType>::RustType,
            referenceTaskIndex: u32,
            operator: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, slashOperatorCall, N> {
            self.call_builder(&slashOperatorCall {
                task,
                referenceTaskIndex,
                operator,
            })
        }
        ///Creates a new call builder for the [`stakeRegistry`] function.
        pub fn stakeRegistry(&self) -> alloy_contract::SolCallBuilder<T, &P, stakeRegistryCall, N> {
            self.call_builder(&stakeRegistryCall {})
        }
        ///Creates a new call builder for the [`taskWasResponded`] function.
        pub fn taskWasResponded(
            &self,
            _0: u32,
        ) -> alloy_contract::SolCallBuilder<T, &P, taskWasRespondedCall, N> {
            self.call_builder(&taskWasRespondedCall { _0 })
        }
        ///Creates a new call builder for the [`transferOwnership`] function.
        pub fn transferOwnership(
            &self,
            newOwner: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, transferOwnershipCall, N> {
            self.call_builder(&transferOwnershipCall { newOwner })
        }
        ///Creates a new call builder for the [`updateAVSMetadataURI`] function.
        pub fn updateAVSMetadataURI(
            &self,
            _metadataURI: alloy::sol_types::private::String,
        ) -> alloy_contract::SolCallBuilder<T, &P, updateAVSMetadataURICall, N> {
            self.call_builder(&updateAVSMetadataURICall { _metadataURI })
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
            T: alloy_contract::private::Transport + ::core::clone::Clone,
            P: alloy_contract::private::Provider<T, N>,
            N: alloy_contract::private::Network,
        > HelloWorldServiceManagerInstance<T, P, N>
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
        ///Creates a new event filter for the [`NewTaskCreated`] event.
        pub fn NewTaskCreated_filter(&self) -> alloy_contract::Event<T, &P, NewTaskCreated, N> {
            self.event_filter::<NewTaskCreated>()
        }
        ///Creates a new event filter for the [`OwnershipTransferred`] event.
        pub fn OwnershipTransferred_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, OwnershipTransferred, N> {
            self.event_filter::<OwnershipTransferred>()
        }
        ///Creates a new event filter for the [`RewardsInitiatorUpdated`] event.
        pub fn RewardsInitiatorUpdated_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, RewardsInitiatorUpdated, N> {
            self.event_filter::<RewardsInitiatorUpdated>()
        }
        ///Creates a new event filter for the [`TaskResponded`] event.
        pub fn TaskResponded_filter(&self) -> alloy_contract::Event<T, &P, TaskResponded, N> {
            self.event_filter::<TaskResponded>()
        }
    }
}
