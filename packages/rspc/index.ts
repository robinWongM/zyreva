// This file was generated by [rspc](https://github.com/oscartbeaumont/rspc). Do not edit this file manually.

export type Procedures = {
    queries: 
        { key: "version", input: never, result: string },
    mutations: 
        { key: "apps.create", input: CreateReq, result: string },
    subscriptions: never
};

export type CreateReq = { git_url: string }
