// This file was generated by [rspc](https://github.com/oscartbeaumont/rspc). Do not edit this file manually.

export type Procedures = {
    queries: 
        { key: "apps.analyze", input: AnalyzeRequest, result: string } | 
        { key: "apps.list", input: never, result: Model[] } | 
        { key: "version", input: never, result: string },
    mutations: 
        { key: "apps.create", input: CreateReq, result: number },
    subscriptions: 
        { key: "apps.build", input: AnalyzeRequest, result: string }
};

export type Model = { id: number; name: string; git_url: string }

export type AnalyzeRequest = { id: number }

export type CreateReq = { git_url: string }
