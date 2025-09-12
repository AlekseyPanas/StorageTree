export enum LicenseLevel {
    BASIC,
    PREMIUM
}

export interface LicenseToken {
    user_id: string;
    level: LicenseLevel;
    expiry_unix: number;
}

export interface IAuthServer {
    fetch_new_license_jwt(session_token: string): Promise<string>;  // return string JWT of LicenseToken type
    submit_login(provider_token: any, provider: string): Promise<string>;  // return session token
    // TODO: Add more as needed
}
