use jwt_simple::prelude::*;
use base64::prelude::*;



// https://github.com/noguxun/ecp/blob/main/recipe_jwt/src/main.rs
// https://github.com/seanpianka/jwt-simple-jwks/blob/master/src/keyset.rs
// https://docs.google.com/document/d/1oq0_RSOHmSiRomeVLfne6YH2gaJavjEAqv-4X7s9DG4/edit?tab=t.0
// https://raw.githubusercontent.com/jfbilodeau/jwks-client/0.1.8/test/test-jwks.json
// https://ingka-icow-prod.eu.auth0.com/.well-known/jwks.json




pub fn jwks() {    
    /*
       
        some internet
        https://github.com/seanpianka/jwt-simple-jwks/blob/master/src/keyset.rs
        https://github.com/seanpianka/jwt-simple-jwks/blob/master/examples/simple.rs
        https://raw.githubusercontent.com/jfbilodeau/jwks-client/0.1.8/test/test-jwks.json
        {
        "e": "AQAB",
        "alg": "RS256",
        "use": "sig",
        "kid": "1",
        "kty": "RSA",
        "n": "t5N44H1mpb5Wlx_0e7CdoKTY8xt-3yMby8BgNdagVNkeCkZ4pRbmQXRWNC7qn__Zaxx9dnzHbzGCul5W0RLfd3oB3PESwsrQh-oiXVEPTYhvUPQkX0vBfCXJtg_zY2mY1DxKOIiXnZ8PaK_7Sx0aMmvR__0Yy2a5dIAWCmjPsxn-PcGZOkVUm-D5bH1-ZStcA_68r4ZSPix7Szhgl1RoHb9Q6JSekyZqM0Qfwhgb7srZVXC_9_m5PEx9wMVNYpYJBrXhD5IQm9RzE9oJS8T-Ai-4_5mNTNXI8f1rrYgffWS4wf9cvsEihrvEg9867B2f98L7ux9Llle7jsHCtwgV1w"
        } 
    
    struct CustomClaims {
        auth_time: i64,
        name: String,
        user_id: String,
        email: String,
    }
    let n_base64 = "t5N44H1mpb5Wlx_0e7CdoKTY8xt-3yMby8BgNdagVNkeCkZ4pRbmQXRWNC7qn__Zaxx9dnzHbzGCul5W0RLfd3oB3PESwsrQh-oiXVEPTYhvUPQkX0vBfCXJtg_zY2mY1DxKOIiXnZ8PaK_7Sx0aMmvR__0Yy2a5dIAWCmjPsxn-PcGZOkVUm-D5bH1-ZStcA_68r4ZSPix7Szhgl1RoHb9Q6JSekyZqM0Qfwhgb7srZVXC_9_m5PEx9wMVNYpYJBrXhD5IQm9RzE9oJS8T-Ai-4_5mNTNXI8f1rrYgffWS4wf9cvsEihrvEg9867B2f98L7ux9Llle7jsHCtwgV1w";
    let e_base64 = "AQAB";
    let token = "eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCIsImtpZCI6IjEifQ.eyJuYW1lIjoiQWRhIExvdmVsYWNlIiwiaXNzIjoiaHR0cHM6Ly9jaHJvbm9nZWFycy5jb20vdGVzdCIsImF1ZCI6InRlc3QiLCJhdXRoX3RpbWUiOjEwMCwidXNlcl9pZCI6InVpZDEyMyIsInN1YiI6InNidTEyMyIsImlhdCI6MjAwLCJleHAiOjUwMCwibmJmIjozMDAsImVtYWlsIjoiYWxvdmVsYWNlQGNocm9ub2dlYXJzLmNvbSJ9.eTQnwXrri_uY55fS4IygseBzzbosDM1hP153EZXzNlLH5s29kdlGt2mL_KIjYmQa8hmptt9RwKJHBtw6l4KFHvIcuif86Ix-iI2fCpqNnKyGZfgERV51NXk1THkgWj0GQB6X5cvOoFIdHa9XvgPl_rVmzXSUYDgkhd2t01FOjQeeT6OL2d9KdlQHJqAsvvKVc3wnaYYoSqv2z0IluvK93Tk1dUBU2yWXH34nX3GAVGvIoFoNRiiFfZwFlnz78G0b2fQV7B5g5F8XlNRdD1xmVZXU8X2-xh9LqRpnEakdhecciFHg0u6AyC4c00rlo_HBb69wlXajQ3R4y26Kpxn7HA";

    use std::time::{SystemTime, UNIX_EPOCH};
    let epoch_seconds = SystemTime::now().duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();
    let option = VerificationOptions{
        time_tolerance: Some(Duration::from_secs(epoch_seconds - 400)),
        ..Default::default()
    };
    */

    /*
        From Kake's test account
        // https://dev-4l22bsbzfmmhqpco.us.auth0.com/.well-known/jwks.json
    */ 

    #[derive(Serialize, Deserialize, Debug)]
    struct CustomClaims {
        sid: String,
    }

    let n_base64 = "sJKoUd7rMwYlI5cXoklPWEIK88vBld2VQiL8Xt5WGzfWyT7F6gczcotGYWipJv9j7eXOGIKGdu-UcAM63hob1g7AV3PwqDTs48DCRzdHyCeQnOPJ7JkHHdetj8Gc72kTI2tdq7PTEBEBgxBJdD3BC378QES3RkjO4woX-ydIp6hXenUYB8G6snIrU96hyC6MRV1LUyGA-K1eprxHixc_14HQS78KCoE9dQzSrb3tpSPXf3AaxBAk5V1-eUz_3our6OHO0ZLCNmReC26f_mfVdjT1dfMHytwB84qXrEJDbqJTZPbKMeMnloOWJddwdb20JEznnsEqWypOd29trzZ1Xw";
    let e_base64 = "AQAB";
    let token = "eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCIsImtpZCI6Il9iTmc4ZUZJT3FFTDEwbFZZT2dBVCJ9.eyJpc3MiOiJodHRwczovL2Rldi00bDIyYnNiemZtbWhxcGNvLnVzLmF1dGgwLmNvbS8iLCJhdWQiOiJib1VpcFBHWWlENXdkRUhBclhyb0RyN1VkTTEzQXhlaCIsImlhdCI6MTczMTY0OTQ0NSwiZXhwIjoxNzMxNjg1NDQ1LCJzdWIiOiJhdXRoMHw2NzM2ZGVhZmQ0Y2Q3YWJmYTE0NTQ2YjciLCJzaWQiOiJnaS1FaWJSckQ0TGljNjJfOTk1cks4b01lZ3ZzTFlzRCIsIm5vbmNlIjoiU2lCVmFCY24wWFRTZW9rSElxOW5LTkRKSTMxbW03N1gifQ.cWpJm0RPVtbhijDSSwPS9W9XGKNei-qji19R3myu25E1gK79U2WhTlGH4BFSjG3FFvPPnGXojrt9TOzdln-89hAgiRB5JTIpXM-acXO1LUTybOSLLuypBtYWCBqwc46JDmJHFQx41bcZ3QmBwDv65xmUxoZmHhHx-kgC-JhNLrY2ri8-6lfdaIYh5FzQnVuayg2o5mpMNTuGmg4JonhUmLOHNw9t7s3CMPPwuH4nvfkfjEr5em6I1IJtZojW0B9klvm6MJ1STPYTZaaTtsA8cxhBamufqQZWQGbtdgcLgBg8gEe5rnJ2TFsWtBvZPdYRhbYBG3VIGrHthvVrs5kAOg";


    let n = BASE64_URL_SAFE_NO_PAD.decode(n_base64).unwrap();
    let e = BASE64_URL_SAFE_NO_PAD.decode(e_base64).unwrap();

    let public_key = RS256PublicKey::from_components(n.as_ref(), e.as_ref()).unwrap();

    // An ugly hack to pass the validation work, this token exp at 500
    
    let option = VerificationOptions{
        time_tolerance: Some(Duration::from_days(365 * 10)),
        ..Default::default()
    };

    match public_key.verify_token::<CustomClaims>(token, Some(option)) {
        Ok(claims) => {
            println!("iss={}", claims.issuer.unwrap());
            println!("sid={}", claims.custom.sid);
        }
        Err(e) => {
            eprintln!("Could not verify token. Reason: {} ", e);
        }
    }
}
