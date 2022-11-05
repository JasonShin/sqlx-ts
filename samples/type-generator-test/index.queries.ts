

export interface ISomeQueryParams {
    
};


export interface ISomeQueryResult {
    id: number;
testName: boolean;
};


export interface ISomeQueryQuery {
    params: ISomeQueryParams;
    result: ISomeQueryResult;
};




export interface ITestQueryWithAliasAndJoinParams {
    
};


export interface ITestQueryWithAliasAndJoinResult {
    number_aliased: number;
id_aliased: number;
};


export interface ITestQueryWithAliasAndJoinQuery {
    params: ITestQueryWithAliasAndJoinParams;
    result: ITestQueryWithAliasAndJoinResult;
};

