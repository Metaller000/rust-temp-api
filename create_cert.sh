CANAME=LocalRootCA
MYCERT=srv
PASS=localhost

mkdir cert 
cd ./cert

openssl genrsa -aes256 -passout pass:$PASS -out $CANAME.key 4096
openssl req -x509 -new -nodes -key $CANAME.key -passin pass:$PASS -sha256 -days 365 -out $CANAME.crt -subj '/CN=Local Root CA/C=RU/ST=Moscow/L=Moscow/O=LocalOrg'
openssl req -new -nodes -out $MYCERT.cer -newkey rsa:4096 -keyout $MYCERT.key -subj '/CN=Local srv/C=RU/ST=Moscow/L=Moscow/O=LocalOrg'

cat > $MYCERT.v3.ext << EOF
authorityKeyIdentifier=keyid,issuer
basicConstraints=CA:FALSE
keyUsage = digitalSignature, nonRepudiation, keyEncipherment, dataEncipherment
subjectAltName = @alt_names
[alt_names]
DNS.1 = localhost
IP.1 = 127.0.0.1
EOF

openssl x509 -req -in $MYCERT.cer -CA $CANAME.crt -passin pass:$PASS -CAkey $CANAME.key -CAcreateserial -out $MYCERT.cer -days 365 -sha256 -extfile $MYCERT.v3.ext
rm -f $MYCERT.v3.ext
cp srv* ../

echo "add root cert to system or brawser"