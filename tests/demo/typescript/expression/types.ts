import { sql } from 'sqlx-ts'

///////////////////////////////////
// Interface, type, enum, module //
///////////////////////////////////

interface TestInterface {
    name: string;
}

type TestType = number;

enum TestEnum {
    a,
    b,
    c,
}

module TestModule {
}


interface TestInterface {
    sql1: string
    get sql2(): string
    set sql3(value: string)
}

module TestModule {
    const moduleSql = sql`SELECT id FROM items`
}

let name: any = 'test'
let companyName = <string>name
let partnerName = name as string
let someName = 'test' as const
