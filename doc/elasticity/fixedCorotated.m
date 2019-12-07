function P = fixedCorotated( F, mu, lambda )
% fixed corotated 2d

[u,sigma,v]=polarSVD(F);
R=u*v';

J=det(F);
A=zeros(2,2);
A(1,1)=F(2,2);
A(2,1)=-F(1,2);
A(1,2)=-F(2,1);
A(2,2)=F(1,1);

P=2*mu*(F-R)+lambda*(J-1)*A;

end



