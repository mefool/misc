function [O1, O2, O3, O4, O5, O6] = inverse_kinematics(A1, A2, A3, A4, A5, A6)
% returns the values of each tetas for up to 8 solutions, having as input
% the position and orientation of the end effector 

%input
px = A1;
py = A2;
pz = A3;
alpha = A4;
beta = A5;
gamma = A6;

% output inicialization
O1 = []; O2 = []; O3 = [];
O4 = []; O5 = []; O6 = [];

% auxiliry arrays
tetas_prem = zeros(8,6);
prem = zeros(1,8);

% orientation/rotation matrix of the end-effector frame relative to the world
% note: the orientation is described by 3 euler angles according to the ZYZ convention.
r_alpha = [cos(alpha), -sin(alpha), 0;
           sin(alpha),  cos(alpha), 0;
           0         ,  0         , 1];

r_beta = [ cos(beta), 0, sin(beta);
           0        , 1, 0        ;
          -sin(beta), 0, cos(beta)];

r_gamma = [cos(gamma), -sin(gamma), 0;
           sin(gamma),  cos(gamma), 0;
           0         ,  0         , 1];

r_0_6 = r_alpha*r_beta*r_gamma;
r_0_6(abs(r_0_6) < 10^-14) = 0;

% transformation matrix of the end-effector frame relative to the world 
t_complete = r_0_6;
t_complete(1,4) = px;
t_complete(2,4) = py;
t_complete(3,4) = pz;
t_complete(4,4) = 1;

% error and step parameters
error = 10^-3;
step = .0005*pi;
%
if (norm([px;py]) < 0.1)
  warning('Near singularity: {norm([px; pz]) approx. 0}')
end

% teta_6 asumption and search solutions according to each assumption
for teta_6_asm = 0:step:2*pi
  
  r_5_6 = [ cos(teta_6_asm), 0, sin(teta_6_asm);
                          0, 1,               0;
           -sin(teta_6_asm), 0, cos(teta_6_asm)];
  
  r_0_5 = r_0_6 * ((r_5_6).');
  
  % point 5
  x_5 = [px; py; pz] - r_0_5*[0; 23.7; -5.5];

  % change of reference frame for the geometric method
  x_m = x_5 - [0; 0; 80]; % from the world frame to frame 2
  
  % teta1
  teta_1 = atan2(x_5(2), x_5(1)) - pi/2;
  teta_1_alt = teta_1 + pi;

  % teta_3
  a2 = 210;
  a3 = sqrt((30)^2+(41.5+180)^2);
  c3 = (x_m(1)^2 + x_m(2)^2 + x_m(3)^2 -(a2)^2 - (a3)^2) / (2*a2*a3);
  if (abs(c3) > 1)
    break
  end

  teta_3 = -1*atan2( +sqrt(1-c3^2), c3) + atan2(41.5+180, 30);
  teta_3_alt = -1*atan2( -sqrt(1-c3^2), c3) + atan2(41.5+180, 30);

  % teta2
  epsilon = atan2(x_m(3), sqrt(x_m(1)^2 + x_m(2)^2));
  delta = acos(((a2)^2 - (a3)^2 + x_m(1)^2 + x_m(2)^2 + x_m(3)^2) / (2*(a2)*sqrt(x_m(1)^2 + x_m(2)^2 + x_m(3)^2)));

  teta_2 = -pi/2 + delta + epsilon;
  teta_2_alt = -pi/2 - delta + epsilon;

  % solutions for the first 3 joints
  tetas = [teta_1    , teta_2     , teta_3    ;
           teta_1    , teta_2_alt , teta_3_alt;
           teta_1_alt, -teta_2    , teta_3_alt;
           teta_1_alt, -teta_2_alt, teta_3    ];
  
  % note: the values of the first 3 tetas, are the same in the first 4 
  % and last 4 solutions.
  tetas = [tetas; 
           tetas];
  
  % for each solution for the first 3 joints, we solve analytically for the
  % other 3 joints. Since we have two solutions (for each combination of 
  % the first 3 tetas) for the last 3 joints we end up with 8 solutions.
  for i = 1:4
    % obtain t_0_3 from each solution for the first 3 joints
    teta_1 = tetas(i,1);
    teta_2 = tetas(i,2);
    teta_3 = tetas(i,3);

    t_0_1 = [cos(teta_1), -sin(teta_1), 0,  0;
             sin(teta_1),  cos(teta_1), 0,  0;
                       0,            0, 1,  0;
                       0,            0, 0,  1];

    t_1_2 = [1,           0,            0,  0;
             0, cos(teta_2), -sin(teta_2),  0;
             0, sin(teta_2),  cos(teta_2), 80;
             0,           0,            0,  1];

    t_2_3 = [1,           0,            0,   0;
             0, cos(teta_3), -sin(teta_3),   0;
             0, sin(teta_3),  cos(teta_3), 210;
             0,           0,            0,   1];

    t_0_3 = t_0_1 * t_1_2 * t_2_3;

    % obtain t_3_6 from t_0_3 and t_complete (t_complete = t_0_6)
    t_3_6 = (t_0_3)^-1 * t_complete;
    
    % teta_5
    teta_5 = acos(t_3_6(2,2));

    if (abs(sin(teta_5)) > 10^-7) % sin(teta_5) != 0
      teta_4 = atan2(t_3_6(1,2), t_3_6(3,2));
      teta_6 = atan2(-t_3_6(2,1), t_3_6(2,3));
    else % sin(teta_5) = 0
      if (abs(cos(teta_5) - 1) < 10^-7) % cos(teta_5) = +1
        teta_4_plus_6 = acos(t_3_6(1,1));
        teta_4 = asin(-2/11*t_3_6(1,4));
        teta_6 = teta_4_plus_6 - teta_4;
      else % cos(teta_5) = -1
        teta_4_minus_6 = acos(t_3_6(1,1));
        teta_4 = asin(2/11*t_3_6(1,4));
        teta_6 = teta_4 - teta_4_minus_6;
      end
    end
    
    % first 4 solutions
    tetas(i,4) = teta_4;
    tetas(i,5) = teta_5;
    tetas(i,6) = teta_6+pi;
    
    % last 4 solutions
    tetas(i+4,4) = teta_4+pi;
    tetas(i+4,5) = -teta_5;
    tetas(i+4,6) = teta_6;
    
    % first 4 solutions confirmation
    dteta_6 = mod(teta_6_asm,2*pi) - mod(teta_6+pi,2*pi);
    if(abs(dteta_6) < error)
      tetas_prem(i,:) = tetas(i,:);
      prem(i) = 1;
    end
    
    %last 4 solutions confirmation
    dteta_6 = mod(teta_6_asm,2*pi) - mod(teta_6,2*pi);
    if(abs(dteta_6) < error)
      tetas_prem(i+4,:) = tetas(i+4,:);
      prem(i+4) = 1;
    end
  end
  
  % if all 8 solutions are found, we can stop the search for the solutions
  if (all(prem))
    break
  end
  
end

% make all tetas be in the interval [0, 2*pi[
tetas_prem = mod(tetas_prem,2*pi);
% remove from the auxiliary array the rows where no solution was found
tetas_prem = tetas_prem(prem == 1,:); 

if (~any(prem))
  warning('No solutions were found.')
  return
end

% output
O1 = tetas_prem(:,1);
O2 = tetas_prem(:,2);
O3 = tetas_prem(:,3);
O4 = tetas_prem(:,4);
O5 = tetas_prem(:,5);
O6 = tetas_prem(:,6);
end
