// @generated automatically by Rapid-web (https://rapid.cincinnati.ventures). DO NOT CHANGE OR EDIT THIS FILE!

export interface Handlers {
	queries: {
		"index": {
  			output: any
  			type: 'query'
  			isDynamic: false
		},
	},
	mutations: {},
}

export const routes = {
	"index": {
		url: '/',
		type: 'query',
	},
} as const

export type DatabaseConnection = Pool;

export interface Database {
    url: string;
}

export interface Post {
    id: number;
    title: string;
    body: string;
    published: boolean;
}