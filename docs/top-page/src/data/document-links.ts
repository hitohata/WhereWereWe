export interface DocumentCategory {
    categoryName: string;
    documentLinks: DocumentLink[];
}
export interface DocumentLink {
    url: string;
    title: string;
    description: string;
}

export const projectDocs: DocumentCategory = {
    categoryName: 'Project Document',
    documentLinks: [
        {
            url: '/project/index.html',
            title: 'Project Document',
            description: 'Project Document'
        }
    ]
}

export const backendDocs: DocumentCategory = {
    categoryName: 'Backend Core Document',
    documentLinks: [
        {
            url: '/core-where-were-we/core_where_were_we/index.html',
            title: 'Core Where Were We',
            description: 'Where Were We Top'
        },
        {
            url: '/core-where-were-we/core_where_were_we/users/index.html',
            title: 'Core Where Were We Users',
            description: 'Users Application'
        },
    ]
}
