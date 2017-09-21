// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

const METHOD_EXECUTION_EXECUTE: ::grpcio::Method<super::remote_execution::ExecuteRequest, super::operations::Operation> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.devtools.remoteexecution.v1test.Execution/Execute",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

pub struct ExecutionClient {
    client: ::grpcio::Client,
}

impl ExecutionClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        ExecutionClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn execute_opt(&self, req: super::remote_execution::ExecuteRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::operations::Operation> {
        self.client.unary_call(&METHOD_EXECUTION_EXECUTE, req, opt)
    }

    pub fn execute(&self, req: super::remote_execution::ExecuteRequest) -> ::grpcio::Result<super::operations::Operation> {
        self.execute_opt(req, ::grpcio::CallOption::default())
    }

    pub fn execute_async_opt(&self, req: super::remote_execution::ExecuteRequest, opt: ::grpcio::CallOption) -> ::grpcio::ClientUnaryReceiver<super::operations::Operation> {
        self.client.unary_call_async(&METHOD_EXECUTION_EXECUTE, req, opt)
    }

    pub fn execute_async(&self, req: super::remote_execution::ExecuteRequest) -> ::grpcio::ClientUnaryReceiver<super::operations::Operation> {
        self.execute_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait Execution {
    fn execute(&self, ctx: ::grpcio::RpcContext, req: super::remote_execution::ExecuteRequest, sink: ::grpcio::UnarySink<super::operations::Operation>);
}

pub fn create_execution<S: Execution + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_EXECUTION_EXECUTE, move |ctx, req, resp| {
        instance.execute(ctx, req, resp)
    });
    builder.build()
}

const METHOD_ACTION_CACHE_GET_ACTION_RESULT: ::grpcio::Method<super::remote_execution::GetActionResultRequest, super::remote_execution::ActionResult> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.devtools.remoteexecution.v1test.ActionCache/GetActionResult",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_ACTION_CACHE_UPDATE_ACTION_RESULT: ::grpcio::Method<super::remote_execution::UpdateActionResultRequest, super::remote_execution::ActionResult> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.devtools.remoteexecution.v1test.ActionCache/UpdateActionResult",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

pub struct ActionCacheClient {
    client: ::grpcio::Client,
}

impl ActionCacheClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        ActionCacheClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn get_action_result_opt(&self, req: super::remote_execution::GetActionResultRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::remote_execution::ActionResult> {
        self.client.unary_call(&METHOD_ACTION_CACHE_GET_ACTION_RESULT, req, opt)
    }

    pub fn get_action_result(&self, req: super::remote_execution::GetActionResultRequest) -> ::grpcio::Result<super::remote_execution::ActionResult> {
        self.get_action_result_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_action_result_async_opt(&self, req: super::remote_execution::GetActionResultRequest, opt: ::grpcio::CallOption) -> ::grpcio::ClientUnaryReceiver<super::remote_execution::ActionResult> {
        self.client.unary_call_async(&METHOD_ACTION_CACHE_GET_ACTION_RESULT, req, opt)
    }

    pub fn get_action_result_async(&self, req: super::remote_execution::GetActionResultRequest) -> ::grpcio::ClientUnaryReceiver<super::remote_execution::ActionResult> {
        self.get_action_result_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn update_action_result_opt(&self, req: super::remote_execution::UpdateActionResultRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::remote_execution::ActionResult> {
        self.client.unary_call(&METHOD_ACTION_CACHE_UPDATE_ACTION_RESULT, req, opt)
    }

    pub fn update_action_result(&self, req: super::remote_execution::UpdateActionResultRequest) -> ::grpcio::Result<super::remote_execution::ActionResult> {
        self.update_action_result_opt(req, ::grpcio::CallOption::default())
    }

    pub fn update_action_result_async_opt(&self, req: super::remote_execution::UpdateActionResultRequest, opt: ::grpcio::CallOption) -> ::grpcio::ClientUnaryReceiver<super::remote_execution::ActionResult> {
        self.client.unary_call_async(&METHOD_ACTION_CACHE_UPDATE_ACTION_RESULT, req, opt)
    }

    pub fn update_action_result_async(&self, req: super::remote_execution::UpdateActionResultRequest) -> ::grpcio::ClientUnaryReceiver<super::remote_execution::ActionResult> {
        self.update_action_result_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait ActionCache {
    fn get_action_result(&self, ctx: ::grpcio::RpcContext, req: super::remote_execution::GetActionResultRequest, sink: ::grpcio::UnarySink<super::remote_execution::ActionResult>);
    fn update_action_result(&self, ctx: ::grpcio::RpcContext, req: super::remote_execution::UpdateActionResultRequest, sink: ::grpcio::UnarySink<super::remote_execution::ActionResult>);
}

pub fn create_action_cache<S: ActionCache + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_ACTION_CACHE_GET_ACTION_RESULT, move |ctx, req, resp| {
        instance.get_action_result(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_ACTION_CACHE_UPDATE_ACTION_RESULT, move |ctx, req, resp| {
        instance.update_action_result(ctx, req, resp)
    });
    builder.build()
}

const METHOD_CONTENT_ADDRESSABLE_STORAGE_FIND_MISSING_BLOBS: ::grpcio::Method<super::remote_execution::FindMissingBlobsRequest, super::remote_execution::FindMissingBlobsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.devtools.remoteexecution.v1test.ContentAddressableStorage/FindMissingBlobs",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_CONTENT_ADDRESSABLE_STORAGE_BATCH_UPDATE_BLOBS: ::grpcio::Method<super::remote_execution::BatchUpdateBlobsRequest, super::remote_execution::BatchUpdateBlobsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.devtools.remoteexecution.v1test.ContentAddressableStorage/BatchUpdateBlobs",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_CONTENT_ADDRESSABLE_STORAGE_GET_TREE: ::grpcio::Method<super::remote_execution::GetTreeRequest, super::remote_execution::GetTreeResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.devtools.remoteexecution.v1test.ContentAddressableStorage/GetTree",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

pub struct ContentAddressableStorageClient {
    client: ::grpcio::Client,
}

impl ContentAddressableStorageClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        ContentAddressableStorageClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn find_missing_blobs_opt(&self, req: super::remote_execution::FindMissingBlobsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::remote_execution::FindMissingBlobsResponse> {
        self.client.unary_call(&METHOD_CONTENT_ADDRESSABLE_STORAGE_FIND_MISSING_BLOBS, req, opt)
    }

    pub fn find_missing_blobs(&self, req: super::remote_execution::FindMissingBlobsRequest) -> ::grpcio::Result<super::remote_execution::FindMissingBlobsResponse> {
        self.find_missing_blobs_opt(req, ::grpcio::CallOption::default())
    }

    pub fn find_missing_blobs_async_opt(&self, req: super::remote_execution::FindMissingBlobsRequest, opt: ::grpcio::CallOption) -> ::grpcio::ClientUnaryReceiver<super::remote_execution::FindMissingBlobsResponse> {
        self.client.unary_call_async(&METHOD_CONTENT_ADDRESSABLE_STORAGE_FIND_MISSING_BLOBS, req, opt)
    }

    pub fn find_missing_blobs_async(&self, req: super::remote_execution::FindMissingBlobsRequest) -> ::grpcio::ClientUnaryReceiver<super::remote_execution::FindMissingBlobsResponse> {
        self.find_missing_blobs_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn batch_update_blobs_opt(&self, req: super::remote_execution::BatchUpdateBlobsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::remote_execution::BatchUpdateBlobsResponse> {
        self.client.unary_call(&METHOD_CONTENT_ADDRESSABLE_STORAGE_BATCH_UPDATE_BLOBS, req, opt)
    }

    pub fn batch_update_blobs(&self, req: super::remote_execution::BatchUpdateBlobsRequest) -> ::grpcio::Result<super::remote_execution::BatchUpdateBlobsResponse> {
        self.batch_update_blobs_opt(req, ::grpcio::CallOption::default())
    }

    pub fn batch_update_blobs_async_opt(&self, req: super::remote_execution::BatchUpdateBlobsRequest, opt: ::grpcio::CallOption) -> ::grpcio::ClientUnaryReceiver<super::remote_execution::BatchUpdateBlobsResponse> {
        self.client.unary_call_async(&METHOD_CONTENT_ADDRESSABLE_STORAGE_BATCH_UPDATE_BLOBS, req, opt)
    }

    pub fn batch_update_blobs_async(&self, req: super::remote_execution::BatchUpdateBlobsRequest) -> ::grpcio::ClientUnaryReceiver<super::remote_execution::BatchUpdateBlobsResponse> {
        self.batch_update_blobs_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_tree_opt(&self, req: super::remote_execution::GetTreeRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::remote_execution::GetTreeResponse> {
        self.client.unary_call(&METHOD_CONTENT_ADDRESSABLE_STORAGE_GET_TREE, req, opt)
    }

    pub fn get_tree(&self, req: super::remote_execution::GetTreeRequest) -> ::grpcio::Result<super::remote_execution::GetTreeResponse> {
        self.get_tree_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_tree_async_opt(&self, req: super::remote_execution::GetTreeRequest, opt: ::grpcio::CallOption) -> ::grpcio::ClientUnaryReceiver<super::remote_execution::GetTreeResponse> {
        self.client.unary_call_async(&METHOD_CONTENT_ADDRESSABLE_STORAGE_GET_TREE, req, opt)
    }

    pub fn get_tree_async(&self, req: super::remote_execution::GetTreeRequest) -> ::grpcio::ClientUnaryReceiver<super::remote_execution::GetTreeResponse> {
        self.get_tree_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait ContentAddressableStorage {
    fn find_missing_blobs(&self, ctx: ::grpcio::RpcContext, req: super::remote_execution::FindMissingBlobsRequest, sink: ::grpcio::UnarySink<super::remote_execution::FindMissingBlobsResponse>);
    fn batch_update_blobs(&self, ctx: ::grpcio::RpcContext, req: super::remote_execution::BatchUpdateBlobsRequest, sink: ::grpcio::UnarySink<super::remote_execution::BatchUpdateBlobsResponse>);
    fn get_tree(&self, ctx: ::grpcio::RpcContext, req: super::remote_execution::GetTreeRequest, sink: ::grpcio::UnarySink<super::remote_execution::GetTreeResponse>);
}

pub fn create_content_addressable_storage<S: ContentAddressableStorage + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_CONTENT_ADDRESSABLE_STORAGE_FIND_MISSING_BLOBS, move |ctx, req, resp| {
        instance.find_missing_blobs(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_CONTENT_ADDRESSABLE_STORAGE_BATCH_UPDATE_BLOBS, move |ctx, req, resp| {
        instance.batch_update_blobs(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_CONTENT_ADDRESSABLE_STORAGE_GET_TREE, move |ctx, req, resp| {
        instance.get_tree(ctx, req, resp)
    });
    builder.build()
}
