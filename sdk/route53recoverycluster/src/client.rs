// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[derive(Debug)]
pub(crate) struct Handle<
    C = aws_smithy_client::erase::DynConnector,
    M = crate::middleware::DefaultMiddleware,
    R = aws_smithy_client::retry::Standard,
> {
    pub(crate) client: aws_smithy_client::Client<C, M, R>,
    pub(crate) conf: crate::Config,
}

/// Client for Route53 Recovery Cluster
///
/// Client for invoking operations on Route53 Recovery Cluster. Each operation on Route53 Recovery Cluster is a method on this
/// this struct. `.send()` MUST be invoked on the generated operations to dispatch the request to the service.
///
/// # Examples
/// **Constructing a client and invoking an operation**
/// ```rust,no_run
/// # async fn docs() {
///     // create a shared configuration. This can be used & shared between multiple service clients.
///     let shared_config = aws_config::load_from_env().await;
///     let client = aws_sdk_route53recoverycluster::Client::new(&shared_config);
///     // invoke an operation
///     /* let rsp = client
///         .<operation_name>().
///         .<param>("some value")
///         .send().await; */
/// # }
/// ```
/// **Constructing a client with custom configuration**
/// ```rust,no_run
/// use aws_config::RetryConfig;
/// # async fn docs() {
///     let shared_config = aws_config::load_from_env().await;
///     let config = aws_sdk_route53recoverycluster::config::Builder::from(&shared_config)
///         .retry_config(RetryConfig::disabled())
///         .build();
///     let client = aws_sdk_route53recoverycluster::Client::from_conf(config);
/// # }
#[derive(std::fmt::Debug)]
pub struct Client<
    C = aws_smithy_client::erase::DynConnector,
    M = crate::middleware::DefaultMiddleware,
    R = aws_smithy_client::retry::Standard,
> {
    handle: std::sync::Arc<Handle<C, M, R>>,
}

impl<C, M, R> std::clone::Clone for Client<C, M, R> {
    fn clone(&self) -> Self {
        Self {
            handle: self.handle.clone(),
        }
    }
}

#[doc(inline)]
pub use aws_smithy_client::Builder;

impl<C, M, R> From<aws_smithy_client::Client<C, M, R>> for Client<C, M, R> {
    fn from(client: aws_smithy_client::Client<C, M, R>) -> Self {
        Self::with_config(client, crate::Config::builder().build())
    }
}

impl<C, M, R> Client<C, M, R> {
    /// Creates a client with the given service configuration.
    pub fn with_config(client: aws_smithy_client::Client<C, M, R>, conf: crate::Config) -> Self {
        Self {
            handle: std::sync::Arc::new(Handle { client, conf }),
        }
    }

    /// Returns the client's configuration.
    pub fn conf(&self) -> &crate::Config {
        &self.handle.conf
    }
}
impl<C, M, R> Client<C, M, R>
where
    C: aws_smithy_client::bounds::SmithyConnector,
    M: aws_smithy_client::bounds::SmithyMiddleware<C>,
    R: aws_smithy_client::retry::NewRequestPolicy,
{
    /// Constructs a fluent builder for the [`GetRoutingControlState`](crate::client::fluent_builders::GetRoutingControlState) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`routing_control_arn(impl Into<String>)`](crate::client::fluent_builders::GetRoutingControlState::routing_control_arn) / [`set_routing_control_arn(Option<String>)`](crate::client::fluent_builders::GetRoutingControlState::set_routing_control_arn): <p>The Amazon Resource Number (ARN) for the routing control that you want to get the state for.</p>
    /// - On success, responds with [`GetRoutingControlStateOutput`](crate::output::GetRoutingControlStateOutput) with field(s):
    ///   - [`routing_control_arn(Option<String>)`](crate::output::GetRoutingControlStateOutput::routing_control_arn): <p>The Amazon Resource Number (ARN) of the response.</p>
    ///   - [`routing_control_state(Option<RoutingControlState>)`](crate::output::GetRoutingControlStateOutput::routing_control_state): <p>The state of the routing control.</p>
    /// - On failure, responds with [`SdkError<GetRoutingControlStateError>`](crate::error::GetRoutingControlStateError)
    pub fn get_routing_control_state(&self) -> fluent_builders::GetRoutingControlState<C, M, R> {
        fluent_builders::GetRoutingControlState::new(self.handle.clone())
    }
    /// Constructs a fluent builder for the [`UpdateRoutingControlState`](crate::client::fluent_builders::UpdateRoutingControlState) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`routing_control_arn(impl Into<String>)`](crate::client::fluent_builders::UpdateRoutingControlState::routing_control_arn) / [`set_routing_control_arn(Option<String>)`](crate::client::fluent_builders::UpdateRoutingControlState::set_routing_control_arn): <p>The Amazon Resource Number (ARN) for the routing control that you want to update the state for.</p>
    ///   - [`routing_control_state(RoutingControlState)`](crate::client::fluent_builders::UpdateRoutingControlState::routing_control_state) / [`set_routing_control_state(Option<RoutingControlState>)`](crate::client::fluent_builders::UpdateRoutingControlState::set_routing_control_state): <p>The state of the routing control. You can set the value to be On or Off.</p>
    /// - On success, responds with [`UpdateRoutingControlStateOutput`](crate::output::UpdateRoutingControlStateOutput)

    /// - On failure, responds with [`SdkError<UpdateRoutingControlStateError>`](crate::error::UpdateRoutingControlStateError)
    pub fn update_routing_control_state(
        &self,
    ) -> fluent_builders::UpdateRoutingControlState<C, M, R> {
        fluent_builders::UpdateRoutingControlState::new(self.handle.clone())
    }
    /// Constructs a fluent builder for the [`UpdateRoutingControlStates`](crate::client::fluent_builders::UpdateRoutingControlStates) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`update_routing_control_state_entries(Vec<UpdateRoutingControlStateEntry>)`](crate::client::fluent_builders::UpdateRoutingControlStates::update_routing_control_state_entries) / [`set_update_routing_control_state_entries(Option<Vec<UpdateRoutingControlStateEntry>>)`](crate::client::fluent_builders::UpdateRoutingControlStates::set_update_routing_control_state_entries): <p>A set of routing control entries that you want to update.</p>
    /// - On success, responds with [`UpdateRoutingControlStatesOutput`](crate::output::UpdateRoutingControlStatesOutput)

    /// - On failure, responds with [`SdkError<UpdateRoutingControlStatesError>`](crate::error::UpdateRoutingControlStatesError)
    pub fn update_routing_control_states(
        &self,
    ) -> fluent_builders::UpdateRoutingControlStates<C, M, R> {
        fluent_builders::UpdateRoutingControlStates::new(self.handle.clone())
    }
}
pub mod fluent_builders {
    //!
    //! Utilities to ergonomically construct a request to the service.
    //!
    //! Fluent builders are created through the [`Client`](crate::client::Client) by calling
    //! one if its operation methods. After parameters are set using the builder methods,
    //! the `send` method can be called to initiate the request.
    //!
    /// Fluent builder constructing a request to `GetRoutingControlState`.
    ///
    /// <p>Get the state for a routing control. A routing control is a simple on/off switch that you can use to route traffic to cells. When the state is On, traffic flows to a cell. When it's off, traffic does not flow. </p>
    /// <p>Before you can create a routing control, you first must create a cluster to host the control. For more information, see <a href="https://docs.aws.amazon.com/recovery-cluster/latest/api/cluster.html">CreateCluster</a>. Access one of the endpoints for the cluster to get or update the routing control state to redirect traffic.</p>
    /// <p>For more information about working with routing controls, see <a href="https://docs.aws.amazon.com/r53recovery/latest/dg/routing-control.html">Routing control</a> in the Route 53 Application Recovery Controller Developer Guide.</p>
    #[derive(std::clone::Clone, std::fmt::Debug)]
    pub struct GetRoutingControlState<
        C = aws_smithy_client::erase::DynConnector,
        M = crate::middleware::DefaultMiddleware,
        R = aws_smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::get_routing_control_state_input::Builder,
    }
    impl<C, M, R> GetRoutingControlState<C, M, R>
    where
        C: aws_smithy_client::bounds::SmithyConnector,
        M: aws_smithy_client::bounds::SmithyMiddleware<C>,
        R: aws_smithy_client::retry::NewRequestPolicy,
    {
        /// Creates a new `GetRoutingControlState`.
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C, M, R>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        /// Sends the request and returns the response.
        ///
        /// If an error occurs, an `SdkError` will be returned with additional details that
        /// can be matched against.
        ///
        /// By default, any retryable failures will be retried twice. Retry behavior
        /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
        /// set when configuring the client.
        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::GetRoutingControlStateOutput,
            aws_smithy_http::result::SdkError<crate::error::GetRoutingControlStateError>,
        >
        where
            R::Policy: aws_smithy_client::bounds::SmithyRetryPolicy<
                crate::input::GetRoutingControlStateInputOperationOutputAlias,
                crate::output::GetRoutingControlStateOutput,
                crate::error::GetRoutingControlStateError,
                crate::input::GetRoutingControlStateInputOperationRetryAlias,
            >,
        {
            let op = self
                .inner
                .build()
                .map_err(|err| aws_smithy_http::result::SdkError::ConstructionFailure(err.into()))?
                .make_operation(&self.handle.conf)
                .await
                .map_err(|err| {
                    aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
                })?;
            self.handle.client.call(op).await
        }
        /// <p>The Amazon Resource Number (ARN) for the routing control that you want to get the state for.</p>
        pub fn routing_control_arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.routing_control_arn(input.into());
            self
        }
        /// <p>The Amazon Resource Number (ARN) for the routing control that you want to get the state for.</p>
        pub fn set_routing_control_arn(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_routing_control_arn(input);
            self
        }
    }
    /// Fluent builder constructing a request to `UpdateRoutingControlState`.
    ///
    /// <p>Set the state of the routing control to reroute traffic. You can set the value to be On or Off. When the state is On, traffic flows to a cell. When it's off, traffic does not flow.</p>
    /// <p>For more information about working with routing controls, see <a href="https://docs.aws.amazon.com/r53recovery/latest/dg/routing-control.html">Routing control</a> in the Route 53 Application Recovery Controller Developer Guide.</p>
    #[derive(std::clone::Clone, std::fmt::Debug)]
    pub struct UpdateRoutingControlState<
        C = aws_smithy_client::erase::DynConnector,
        M = crate::middleware::DefaultMiddleware,
        R = aws_smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::update_routing_control_state_input::Builder,
    }
    impl<C, M, R> UpdateRoutingControlState<C, M, R>
    where
        C: aws_smithy_client::bounds::SmithyConnector,
        M: aws_smithy_client::bounds::SmithyMiddleware<C>,
        R: aws_smithy_client::retry::NewRequestPolicy,
    {
        /// Creates a new `UpdateRoutingControlState`.
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C, M, R>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        /// Sends the request and returns the response.
        ///
        /// If an error occurs, an `SdkError` will be returned with additional details that
        /// can be matched against.
        ///
        /// By default, any retryable failures will be retried twice. Retry behavior
        /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
        /// set when configuring the client.
        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::UpdateRoutingControlStateOutput,
            aws_smithy_http::result::SdkError<crate::error::UpdateRoutingControlStateError>,
        >
        where
            R::Policy: aws_smithy_client::bounds::SmithyRetryPolicy<
                crate::input::UpdateRoutingControlStateInputOperationOutputAlias,
                crate::output::UpdateRoutingControlStateOutput,
                crate::error::UpdateRoutingControlStateError,
                crate::input::UpdateRoutingControlStateInputOperationRetryAlias,
            >,
        {
            let op = self
                .inner
                .build()
                .map_err(|err| aws_smithy_http::result::SdkError::ConstructionFailure(err.into()))?
                .make_operation(&self.handle.conf)
                .await
                .map_err(|err| {
                    aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
                })?;
            self.handle.client.call(op).await
        }
        /// <p>The Amazon Resource Number (ARN) for the routing control that you want to update the state for.</p>
        pub fn routing_control_arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.routing_control_arn(input.into());
            self
        }
        /// <p>The Amazon Resource Number (ARN) for the routing control that you want to update the state for.</p>
        pub fn set_routing_control_arn(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_routing_control_arn(input);
            self
        }
        /// <p>The state of the routing control. You can set the value to be On or Off.</p>
        pub fn routing_control_state(mut self, input: crate::model::RoutingControlState) -> Self {
            self.inner = self.inner.routing_control_state(input);
            self
        }
        /// <p>The state of the routing control. You can set the value to be On or Off.</p>
        pub fn set_routing_control_state(
            mut self,
            input: std::option::Option<crate::model::RoutingControlState>,
        ) -> Self {
            self.inner = self.inner.set_routing_control_state(input);
            self
        }
    }
    /// Fluent builder constructing a request to `UpdateRoutingControlStates`.
    ///
    /// <p>Set multiple routing control states. You can set the value for each state to be On or Off. When the state is On, traffic flows to a cell. When it's off, traffic does not flow.</p>
    /// <p>For more information about working with routing controls, see <a href="https://docs.aws.amazon.com/r53recovery/latest/dg/routing-control.html">Routing control</a> in the Route 53 Application Recovery Controller Developer Guide.</p>
    #[derive(std::clone::Clone, std::fmt::Debug)]
    pub struct UpdateRoutingControlStates<
        C = aws_smithy_client::erase::DynConnector,
        M = crate::middleware::DefaultMiddleware,
        R = aws_smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::update_routing_control_states_input::Builder,
    }
    impl<C, M, R> UpdateRoutingControlStates<C, M, R>
    where
        C: aws_smithy_client::bounds::SmithyConnector,
        M: aws_smithy_client::bounds::SmithyMiddleware<C>,
        R: aws_smithy_client::retry::NewRequestPolicy,
    {
        /// Creates a new `UpdateRoutingControlStates`.
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C, M, R>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        /// Sends the request and returns the response.
        ///
        /// If an error occurs, an `SdkError` will be returned with additional details that
        /// can be matched against.
        ///
        /// By default, any retryable failures will be retried twice. Retry behavior
        /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
        /// set when configuring the client.
        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::UpdateRoutingControlStatesOutput,
            aws_smithy_http::result::SdkError<crate::error::UpdateRoutingControlStatesError>,
        >
        where
            R::Policy: aws_smithy_client::bounds::SmithyRetryPolicy<
                crate::input::UpdateRoutingControlStatesInputOperationOutputAlias,
                crate::output::UpdateRoutingControlStatesOutput,
                crate::error::UpdateRoutingControlStatesError,
                crate::input::UpdateRoutingControlStatesInputOperationRetryAlias,
            >,
        {
            let op = self
                .inner
                .build()
                .map_err(|err| aws_smithy_http::result::SdkError::ConstructionFailure(err.into()))?
                .make_operation(&self.handle.conf)
                .await
                .map_err(|err| {
                    aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
                })?;
            self.handle.client.call(op).await
        }
        /// Appends an item to `UpdateRoutingControlStateEntries`.
        ///
        /// To override the contents of this collection use [`set_update_routing_control_state_entries`](Self::set_update_routing_control_state_entries).
        ///
        /// <p>A set of routing control entries that you want to update.</p>
        pub fn update_routing_control_state_entries(
            mut self,
            input: crate::model::UpdateRoutingControlStateEntry,
        ) -> Self {
            self.inner = self.inner.update_routing_control_state_entries(input);
            self
        }
        /// <p>A set of routing control entries that you want to update.</p>
        pub fn set_update_routing_control_state_entries(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::UpdateRoutingControlStateEntry>>,
        ) -> Self {
            self.inner = self.inner.set_update_routing_control_state_entries(input);
            self
        }
    }
}

impl<C> Client<C, crate::middleware::DefaultMiddleware, aws_smithy_client::retry::Standard> {
    /// Creates a client with the given service config and connector override.
    pub fn from_conf_conn(conf: crate::Config, conn: C) -> Self {
        let retry_config = conf.retry_config.as_ref().cloned().unwrap_or_default();
        let timeout_config = conf.timeout_config.as_ref().cloned().unwrap_or_default();
        let sleep_impl = conf.sleep_impl.clone();
        let mut builder = aws_smithy_client::Builder::new()
            .connector(conn)
            .middleware(crate::middleware::DefaultMiddleware::new());
        builder.set_retry_config(retry_config.into());
        builder.set_timeout_config(timeout_config);
        if let Some(sleep_impl) = sleep_impl {
            builder.set_sleep_impl(Some(sleep_impl));
        }
        let client = builder.build();
        Self {
            handle: std::sync::Arc::new(Handle { client, conf }),
        }
    }
}
impl
    Client<
        aws_smithy_client::erase::DynConnector,
        crate::middleware::DefaultMiddleware,
        aws_smithy_client::retry::Standard,
    >
{
    /// Creates a new client from a shared config.
    #[cfg(any(feature = "rustls", feature = "native-tls"))]
    pub fn new(config: &aws_types::config::Config) -> Self {
        Self::from_conf(config.into())
    }

    /// Creates a new client from the service [`Config`](crate::Config).
    #[cfg(any(feature = "rustls", feature = "native-tls"))]
    pub fn from_conf(conf: crate::Config) -> Self {
        let retry_config = conf.retry_config.as_ref().cloned().unwrap_or_default();
        let timeout_config = conf.timeout_config.as_ref().cloned().unwrap_or_default();
        let sleep_impl = conf.sleep_impl.clone();
        let mut builder = aws_smithy_client::Builder::dyn_https()
            .middleware(crate::middleware::DefaultMiddleware::new());
        builder.set_retry_config(retry_config.into());
        builder.set_timeout_config(timeout_config);
        // the builder maintains a try-state. To avoid suppressing the warning when sleep is unset,
        // only set it if we actually have a sleep impl.
        if let Some(sleep_impl) = sleep_impl {
            builder.set_sleep_impl(Some(sleep_impl));
        }
        let client = builder.build();

        Self {
            handle: std::sync::Arc::new(Handle { client, conf }),
        }
    }
}
