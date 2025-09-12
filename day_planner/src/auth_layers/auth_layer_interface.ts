export enum LicenseLevel {
    BASIC,
    PREMIUM
}

export interface AuthLayer {
    is_logged_in(): boolean;
    get_license_level(): LicenseLevel | null;
    submit_id_token(token: any, provider: string): Promise<boolean>;
}
