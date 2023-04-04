wash claims sign bankaccount.linked.wasm \
		 -c cosmonic:eventsourcing  \
		--name "Bank Account Aggregate (Component)" --ver 0.1.0 --rev 1 \
		--issuer `cat ./bank_issuer.nk` --directory '.' --disable-keygen \
		 --destination build/bankaccount.linked.signed.wasm