import {LicenseToken} from "../auth_server/auth_server_interface";


export interface IAuthLayer {
    is_logged_in(): boolean;  // verify local session token and return if it exists and is valid and unexpired
    get_session_token(): string | null;  // null means it doesn't exist yet
    get_license(): LicenseToken | null;  // null means expired/invalid AND could not acquire new one
    submit_id_token(token: any, provider: string): Promise<boolean>;
}
