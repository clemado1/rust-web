let URL: string = 'http://localhost:5000/api/'

/**
 * login -> response
 * logout -> response
 * get_me -> LoggedUser
 * auth -> response
 * register -> response
 */

export const authRequest = async (method: string): Promise<any> => {
    let call_url: string = URL + 'auth'
    try {
        const response = await fetch(call_url, {
            method,
            headers: {
                Accept: 'application/json',
                'Content-Type': 'application/json',
            },
        })

        console.log(response)

        if (response.status !== 404) {
            return await response.json()
        }
    } catch (err) {
        console.log('15')
        console.log(err)
        return undefined
    }
}

export const inviteUser = async (email: string): Promise<any> => {
    let method: string = 'POST'
    let call_url: string = URL + 'invitation'
    const response = await fetch(call_url, {
        method,
        headers: {
            Accept: 'application/json',
            'Content-Type': 'application/json',
        },
        body: JSON.stringify(email),
    })

    return await response
}

export const registerUser = async (
    id: string,
    password: string
): Promise<any> => {
    let method: string = 'POST'
    let call_url: string = URL + 'register/' + id
    const response = await fetch(call_url, {
        method,
        headers: {
            Accept: 'application/json',
            'Content-Type': 'application/json',
        },
        body: JSON.stringify(password),
    })

    return await response
}
