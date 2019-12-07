function [u, sigma, v] = polarSVD(F)
% polar svd 2d

[u,sigma,v] = svd(F);

if(det(u)<0)
   u(1,2)=u(1,2)*-1;
   u(2,2)=u(2,2)*-1;
   sigma(2,2)=sigma(2,2)*-1;
end
if(det(v)<0)
   v(1,2)=v(1,2)*-1;
   v(2,2)=v(2,2)*-1;
   sigma(2,2)=sigma(2,2)*-1;
end

end
